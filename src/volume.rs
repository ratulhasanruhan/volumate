use winmix::WinMix;
use std::error::Error;

/// Get the current master volume (0.0 to 1.0)
pub fn get_master_volume() -> Result<f32, Box<dyn Error>> {
    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate()?;

        // Get the first available session volume
        if let Some(session) = sessions.into_iter().next() {
            let vol = session.vol.get_master_volume()?;
            Ok(vol)
        } else {
            Ok(0.5) // Default if no sessions
        }
    }
}

/// Set the master volume for all audio sessions (0.0 to 1.0)
pub fn set_master_volume(level: f32) -> Result<(), Box<dyn Error>> {
    let level = level.clamp(0.0, 1.0);

    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate()?;

        if sessions.is_empty() {
            return Err("No audio sessions found".into());
        }

        for session in sessions {
            // Unmute if needed
            let _ = session.vol.set_mute(false);

            // Set volume
            if let Err(e) = session.vol.set_master_volume(level) {
                eprintln!("Warning: Failed to set volume for PID {}: {}", session.pid, e);
            } else {
                println!("Set volume for PID {} to {:.0}%", session.pid, level * 100.0);
            }
        }
    }

    Ok(())
}

/// Get mute state
pub fn get_mute() -> Result<bool, Box<dyn Error>> {
    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate()?;

        if let Some(session) = sessions.into_iter().next() {
            let muted = session.vol.get_mute()?;
            Ok(muted)
        } else {
            Ok(false)
        }
    }
}

/// Set mute state for all audio sessions
pub fn set_mute(muted: bool) -> Result<(), Box<dyn Error>> {
    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate()?;

        for session in sessions {
            if let Err(e) = session.vol.set_mute(muted) {
                eprintln!("Warning: Failed to set mute for PID {}: {}", session.pid, e);
            }
        }
    }

    Ok(())
}
