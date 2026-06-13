# A Practical Guide to Configuring a DMR Radio

> **Digital Mobile Radio · Setup Guide**
> DMR gives you crisp digital audio, two conversations on one frequency, and tidy contact management — but the first setup can feel dense. This guide walks through every concept and every step, in plain language, from a blank codeplug to a working radio.

**Level:** Beginner → Intermediate  ·  **Read time:** ~20 min  ·  **You'll need:** a radio, its CPS, a programming cable

---

## Contents

1. [DMR in 5 minutes](#dmr-in-5-minutes)
2. [Core building blocks](#core-building-blocks)
3. [Before you start](#before-you-start)
4. [Step 1 · Set your radio ID](#step-1--set-your-radio-id)
5. [Step 2 · Build your contacts](#step-2--build-your-contacts)
6. [Step 3 · RX group lists](#step-3--rx-group-lists)
7. [Step 4 · Create channels](#step-4--create-channels)
8. [Step 5 · Organize zones](#step-5--organize-zones)
9. [Step 6 · Scan lists](#step-6--scan-lists)
10. [Step 7 · Write to the radio](#step-7--write-to-the-radio)
11. [Making two radios talk](#making-two-radios-talk)
12. [Private calls, done right](#private-calls-done-right)
13. [Troubleshooting](#troubleshooting)
14. [Glossary](#glossary)

---

## DMR in 5 minutes

*What the technology actually does, before we touch any settings.*

**DMR (Digital Mobile Radio)** is an open digital voice standard. Instead of sending your voice as a raw analog signal, the radio digitizes it, packages it, and transmits it in precisely timed bursts. The result is clean audio that stays clear right up to the edge of range, plus features analog simply can't offer — text messaging, caller identification, and addressing calls to a specific person or group.

### The one idea that explains everything: time slots

DMR uses **TDMA** — Time-Division Multiple Access. A single 12.5 kHz channel is sliced into two alternating time slots, **TS1** and **TS2**. Two completely separate conversations can share one frequency without colliding, because each one only transmits during its own slot. This is also why a digital signal can sound like it "cuts in and out" at the fringe — you're hearing the gaps between bursts rather than analog static.

> ℹ️ **Slots need a clock.** On a repeater, the repeater broadcasts the master timing that keeps both slots aligned. Talking radio-to-radio without a repeater (*simplex*) there is no master clock, so direct mode effectively uses a single slot and the radios sync to each other on the fly.

---

## Core building blocks

*Six concepts do almost all the work. Learn these and the programming software stops being intimidating.*

| Term | What it is | Why it matters |
|------|-----------|----------------|
| **Radio ID** | A unique number that identifies your radio on the network. | Every digital transmission carries it. Calls are addressed to and from these numbers. |
| **Color Code** | A value from `0`–`15`, the digital equivalent of a tone squelch. | Both ends *must* use the same color code or they won't hear each other. |
| **Time Slot** | `TS1` or `TS2` — which half of the channel a call uses. | Must match between radios; set by the repeater or network plan. |
| **Talkgroup** | A group address many radios share to talk together. | Stored as a *Group Call* contact and assigned as a channel's transmit target. |
| **Contact** | A saved ID with a name and a call type (Group, Private, or All Call). | Defines *who* a transmission is addressed to. |
| **Channel** | One complete configuration: frequency, slot, color code, contact, and dozens of options. | The basic thing you turn the knob to. Everything else plugs into it. |
| **RX Group List** | The set of talkgroups a channel will *listen* to. | Lets one channel monitor several groups at once. |
| **Zone** | A folder of channels (radios cap how many channels a zone holds). | How you keep dozens of channels organized and reachable. |

> 💡 **The mental model:** A *contact* is who you talk to. A *channel* bundles a frequency with one contact and a pile of settings. A *zone* is a folder of channels. Build them in that order and nothing feels out of sequence.

---

## Before you start

*A few minutes of prep saves a lot of backtracking.*

- **Request a Radio ID.** Amateur operators obtain a free ID from the global registry for ham DMR. Commercial and licensed-business use follows its own licensing and numbering — use the ID assigned to your system.
- **Install the CPS.** The *Customer Programming Software* is the desktop app that edits your radio's configuration. Use the version that matches your model and firmware.
- **Get the right cable and driver.** Many programming cables need a specific USB driver. Confirm the radio appears as a COM port before you try to read or write.
- **Read & back up first.** Connect the radio, *read* its current configuration, and save that file untouched. This saved configuration is your **codeplug** — your safety net if anything goes wrong.
- **Gather your frequency plan.** Have your frequencies, color codes, time slots, and talkgroup numbers written down before you open the software.

> ⚠️ **Transmit only where you're authorized.** Programming a frequency does not grant permission to use it. Stay within the bands, power limits, and privileges your license allows, and follow local regulations.

---

## Step 1 · Set your radio ID

*The single number that identifies you on every digital call.*

In the CPS this lives in the general or DMR settings area, usually labeled *Radio ID* or *DMR ID*. Enter the ID assigned to your radio. Some radios support several IDs in an ID list and let each channel pick which one to use; for a typical setup, a single ID used everywhere is correct and avoids confusion.

> ℹ️ **One radio, one identity.** If your radio offers a per-channel ID override, leave every channel pointed at the same ID unless you have a specific reason not to. A mismatched per-channel ID is a classic source of "it transmits but the other end behaves oddly."

---

## Step 2 · Build your contacts

*Define who you can address before you build the channels that point at them.*

A contact is just a name, an ID number, and a **call type**. Create one entry for every talkgroup and every individual you want to reach.

| Call type | Reaches | Typical use |
|-----------|---------|-------------|
| **Group Call** | Everyone monitoring that talkgroup. | Nets, general chatter, the everyday default. |
| **Private Call** | One specific radio, by its ID. | A direct, one-to-one conversation. |
| **All Call** | Every radio that can hear the signal. | Announcements; use sparingly. |

Give each contact a clear, short name — it's what shows on radios that have a display, and what you'll pick from lists later in the software. Double-check every ID number; a wrong digit means the call is addressed to nobody.

---

## Step 3 · RX group lists

*Decide which groups a channel should listen to, not just talk on.*

A channel transmits to *one* contact, but it can *receive* several talkgroups at once. An **RX group list** is simply a named bundle of group contacts. Build a list that contains every talkgroup you want to hear on a given channel, then attach that list to the channel in the next step.

> 💡 **Keep it lean.** Only put groups you genuinely want to hear into a list. An overstuffed RX list means constant interruptions from traffic you don't care about.

---

## Step 4 · Create channels

*Where everything comes together. This is the table you'll spend the most time in.*

Each channel is either **digital** (DMR) or **analog** (FM). The fields below are the ones that matter on a digital channel; analog channels swap the digital options for tone squelch (CTCSS/DCS).

| Field | What to enter |
|-------|---------------|
| **Name** | A short, recognizable label. This is what you see when you turn the knob. |
| **RX Frequency** | The frequency you listen on. |
| **TX Frequency** | The frequency you transmit on. Same as RX for simplex; offset for a repeater. |
| **Channel Type** | `Digital` for DMR, `Analog` for FM. |
| **Color Code** | Must match the other end / the repeater. `1` is a common default. |
| **Time Slot** | `TS1` or `TS2` per your plan. Direct simplex generally uses slot 1. |
| **TX Contact** | The contact this channel calls when you key up — the heart of the channel. |
| **RX Group List** | The listen list you built in step 3. |
| **Power** | High or low. Use the least power that does the job. |
| **Admit Criteria** | When the radio is allowed to transmit — see below. |
| **TX Timeout (TOT)** | A safety limit that ends a stuck transmission after a set time. |

### Admit criteria, briefly

This setting controls whether the radio checks the channel before transmitting:

- **Always** — transmit immediately, no checking. Fine for simple simplex use.
- **Channel Free** — only transmit if the channel is idle. Polite; avoids stepping on others.
- **Color Code Free** — transmit only if the channel is free *and* carrying your color code. Best on shared repeaters.

> ⚠️ **The three things that must match.** For two digital radios to hear each other, they need the same *frequency*, the same *color code*, and the same *time slot*. Any one of these wrong means silence — even though both radios show they're transmitting.

---

## Step 5 · Organize zones

*Turn a long, flat channel list into something you can actually navigate.*

A **zone** is a folder. Radios limit how many channels a zone can hold and how many you can reach from the front panel, so group channels logically — by location, by purpose, or by group. Add your channels to a zone *in the order you want to scroll through them*; most radios present channels in the sequence they appear in the zone.

> 💡 **Order on purpose.** Put your most-used channels first and keep related channels adjacent. A zone whose channels jump around in number is confusing to scroll; arranging them in a clean, continuous sequence makes the radio far nicer to operate.

---

## Step 6 · Scan lists

*Let the radio watch several channels and stop on whichever one comes alive.*

A **scan list** is a set of channels the radio cycles through, pausing on any that has activity. Build a list, add the channels you want to monitor, optionally flag one or two as *priority* (checked more often), and assign the list to the channels that should scan. Keep scan lists focused — scanning too many channels makes the radio slow to land on the one you care about.

---

## Step 7 · Write to the radio

*Commit your work — carefully.*

1. **Save the file.** Save your edited configuration under a clear, dated name. Keep the original backup from earlier untouched, so you can always roll back.
2. **Connect and confirm.** Plug in the cable and confirm the CPS sees the radio. Match the CPS version to the radio's firmware to avoid a corrupt write.
3. **Write the codeplug.** Use *Write* to send the configuration to the radio. Don't disconnect or power off mid-write.
4. **Test on the bench.** Verify on a known-good channel first: confirm you transmit and receive, then work through the rest before relying on it in the field.

> ℹ️ **Iterate in the file, not on the radio.** Make changes in the CPS, write, test, and repeat. Editing the saved configuration and re-writing is faster and safer than poking settings on the radio itself.

---

## Making two radios talk

*The shortest path to a working contact between two digital radios.*

Strip away the options and a working digital link needs only a handful of things to line up on both radios:

1. Same **RX/TX frequency** (and the same simplex-vs-repeater offset).
2. Same **color code**.
3. Same **time slot**.
4. A **TX contact** that addresses the call where the other radio is listening.
5. Matching **bandwidth** and digital channel type on both ends.

Get those five aligned and audio flows. Everything else — scan, zones, RX lists, power — is convenience layered on top.

> ℹ️ **A note on simplex timing.** Without a repeater providing a master clock, two radios have to sync to each other when a conversation starts cold. The very first transmission after a quiet channel can be partly clipped while they lock on — this is normal for direct-mode digital. Once a conversation is flowing it's seamless. A brief pause between keying up and speaking sidesteps it entirely.

---

## Private calls, done right

*Group calls are easy. Private calls trip almost everyone up — here's the whole picture in one place.*

A **private call** is addressed to *one specific radio* by its unique Radio ID, instead of to a shared group. It's the digital equivalent of phoning a single person rather than announcing to a room. Conceptually simple — but the setup has a few non-obvious steps, and missing any one of them produces the same baffling symptom: your radio keys up, the indicator lights, and the other end stays silent.

> ⚠️ **The one mistake everyone makes.** Creating a private contact does *nothing* on its own. A contact just defines an address; it has no effect until a *channel* is told to transmit to it. The most common reason "private calls don't work" is that the contact exists but no channel actually points at it — so every press still goes out as whatever the channel was already set to.

### Setting one up — the four pieces

1. **Create a Private Call contact.** Add a contact with call type **Private Call** and an ID equal to the *destination radio's* Radio ID — exactly, every digit. This is the address the call is sent to.
2. **Point a channel's TX Contact at it.** Open the channel you'll use and set its **TX Contact** to that private contact. This is the step people miss. The channel — not the contact list — decides who you call.
3. **Confirm your own Radio ID is correct.** Your radio announces itself by its own ID. If that's wrong, the far radio sees a call from the wrong source — and on the return call, *your* radio may not recognize it's being addressed.
4. **Match the radio layer.** Same **frequency**, **color code**, and **time slot** as the other radio — exactly as for any digital contact. A private call obeys these rules too.

### How receiving actually works (the part nobody explains)

This is the detail missing from most guides: **a radio receives a private call automatically whenever the call is addressed to its own Radio ID.** You do *not* add anything to an RX group list to receive private calls — RX group lists govern *talkgroup* (group-call) reception only. As long as your Radio ID is set correctly and the channel's frequency, color code, and slot match, an incoming private call to your ID will open the speaker on its own.

> ℹ️ **Two IDs are always in play.** Every call has a *source* (your radio's own ID) and a *destination* (the TX contact). A private call is delivered to the radio whose own ID equals the destination. So both ends must have their own ID set correctly *and* a contact pointing at the other — it's symmetric: each radio needs a channel addressing the other.

### "It transmits but I hear nothing" — the checklist

The indicator lighting up only proves your radio is transmitting; it says nothing about whether the call was addressed correctly or whether the far radio can decode it. Work down this list:

- **Destination ID** — does the contact's ID exactly match the other radio's Radio ID? A single wrong digit sends the call to nobody.
- **Channel assignment** — is that contact actually set as the channel's TX Contact, not just sitting in the contact list?
- **Your own ID** — is the receiving radio's own Radio ID set to the value being called?
- **Color code & time slot** — both must match, or the receiver's squelch never opens.
- **Frequency & bandwidth** — identical on both ends, both set to digital.

### Private Call Confirmed — the optional handshake

Many radios offer a **Private Call Confirmed** option. When enabled, keying up first sends a short setup request and waits for the other radio to acknowledge before letting you talk:

- A **talk-permit tone** means the link is confirmed — the other radio answered, so it's powered on and in range. Then you speak.
- A **reject tone** means no acknowledgment — the other radio is off, out of range, or busy — and you won't transmit that over.

That makes it a handy presence-and-range check before you rely on the channel. The trade-offs: a brief delay and a tone before each over, and if the other radio can't be reached you simply can't key up. Leaving it off lets you transmit unconditionally, at the cost of no confirmation. Either is valid — pick based on whether you'd rather always be able to talk, or always know the call got through.

> 💡 **On simplex, expect the first over to need a moment.** With no repeater clock, the two radios sync to each other when a conversation starts cold, so the very first transmission can be clipped. The confirmed-call handshake also helps here, since the link is established before your voice begins. Once you're going back and forth, it's seamless.

---

## Troubleshooting

*The usual suspects, and what to check first.*

| Symptom | Most likely cause | Fix |
|---------|-------------------|-----|
| Transmit indicator lights, but the other radio hears nothing. | Color code, time slot, or addressing mismatch — the receiver's squelch never opens. | Confirm color code, slot, and frequency match exactly, and that the call is addressed correctly. |
| First few words of a conversation get cut off. | Normal simplex sync, or a short TX preamble. | Pause briefly after keying up; increasing TX preamble can help. |
| You can transmit but never receive a group's traffic. | That talkgroup isn't in the channel's RX group list. | Add the group to the RX list and reassign it to the channel. |
| Audio sounds robotic or drops in bursts. | Weak signal at the digital edge of range. | Improve antenna/position, raise power, or move closer. |
| The radio won't transmit at all on a channel. | Admit criteria is blocking, or TX is inhibited on that channel. | Check admit criteria and any transmit-prohibit flag. |
| CPS write fails or the radio acts strangely afterward. | CPS/firmware version mismatch. | Match versions, re-read the radio, and write a known-good backup. |

---

## Glossary

| Term | Definition |
|------|------------|
| **Admit Criteria** | The rule that decides whether the radio is allowed to transmit on a channel (always, only when free, or only when free on your color code). |
| **Bandwidth** | The width of the channel. DMR voice uses 12.5 kHz. |
| **Codeplug** | The complete configuration file for a radio — every channel, contact, zone, and setting in one package. |
| **Color Code** | A 0–15 value that acts as a digital squelch; both ends must match to communicate. |
| **CPS** | Customer Programming Software — the desktop app used to edit and write a codeplug. |
| **CTCSS / DCS** | Sub-audible tone and digital-coded squelch used on *analog* channels, the FM analogue of a color code. |
| **Simplex** | Radio-to-radio operation on a single frequency, with no repeater in between. |
| **TDMA** | Time-Division Multiple Access — the scheme that splits one DMR channel into two time slots. |
| **Talkgroup** | A shared group address that lets many radios converse together. |
| **Time Slot (TS1/TS2)** | One of the two interleaved time windows on a DMR channel. |
| **TOT** | Time-Out Timer — ends a transmission automatically after a set duration to prevent a stuck mic. |
| **Zone** | A folder grouping a set of channels for easy navigation on the radio. |

---

*DMR Field Manual · A practical configuration guide. Always operate within your license and local regulations.*
