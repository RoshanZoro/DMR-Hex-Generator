//! Clipboard writes with verified auto-wipe: the wipe only fires if the
//! clipboard still holds the value we wrote, so unrelated content is preserved.

use zeroize::Zeroizing;

pub struct ClipboardManager {
    inner: Option<arboard::Clipboard>,
    /// The last value written, kept to verify ownership before wiping.
    last_set: Option<Zeroizing<String>>,
    init_error: Option<String>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        match arboard::Clipboard::new() {
            Ok(c) => ClipboardManager {
                inner: Some(c),
                last_set: None,
                init_error: None,
            },
            Err(e) => ClipboardManager {
                inner: None,
                last_set: None,
                init_error: Some(e.to_string()),
            },
        }
    }

    pub fn init_error(&self) -> Option<&str> {
        self.init_error.as_deref()
    }

    /// Copy `text` to the clipboard and remember it for later wiping.
    pub fn set(&mut self, text: Zeroizing<String>) -> Result<(), String> {
        let cb = self
            .inner
            .as_mut()
            .ok_or_else(|| "Clipboard unavailable".to_string())?;
        cb.set_text(text.as_str().to_owned())
            .map_err(|e| e.to_string())?;
        self.last_set = Some(text);
        Ok(())
    }

    /// Wipe the clipboard if it still contains the value we last set.
    ///
    /// Returns true if a wipe was performed.
    pub fn wipe_if_ours(&mut self) -> bool {
        let Some(cb) = self.inner.as_mut() else {
            self.last_set = None;
            return false;
        };
        let Some(ours) = self.last_set.take() else {
            return false;
        };

        let still_ours = cb
            .get_text()
            .map(|cur| cur == *ours)
            .unwrap_or(false);

        if still_ours {
            // Overwrite, then clear, so the value is replaced rather than just
            // emptied (defeats some "restore last clip" behaviours).
            let _ = cb.set_text(String::from(" "));
            let _ = cb.clear();
            true
        } else {
            false
        }
    }
}

impl Drop for ClipboardManager {
    fn drop(&mut self) {
        // Best effort so a copied key does not outlive the program.
        self.wipe_if_ours();
    }
}
