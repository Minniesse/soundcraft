//! The engine built without the `plugin-hosts` feature (the KatWork build profile): no CLAP, VST3 or
//! Audio Units host is compiled in, so a hosted plugin id is an unknown plugin and the hosted plugin
//! lists are empty. `cargo test -p soundcraft-engine --no-default-features --test katwork_profile`.
#![cfg(not(feature = "plugin-hosts"))]

use craft_audio::AudioBuffer;
use serde_json::{Value, json};
use soundcraft_engine::{Engine, EngineError};
use soundcraft_model::{ChannelFormat, Clip, Session, SourceAudio, SourceId, TrackKind};
use soundcraft_time::Range;
use std::sync::Arc;

const HOSTED_IDS: [&str; 3] = ["clap:com.example.gain", "vst3:0123456789abcdef0123456789abcdef", "au:aufx:bpas:appl"];

/// An audio track playing a constant signal, with `plugin` (if any) in insert slot 0.
fn session_with_insert(plugin: Option<&str>) -> Session {
    let mut e = Engine::default();
    let s = e.session_mut();
    let t = s.add_track(TrackKind::Audio, ChannelFormat::Mono, None);
    s.pool.insert(SourceId(7), Arc::new(SourceAudio::new(AudioBuffer { sample_rate: 48_000, channels: vec![vec![0.25; 4800]] })));
    let id = s.new_clip_id();
    s.track_mut(t).unwrap().playlist_mut().unwrap().clips.push(Clip::audio(id, "dc", SourceId(7), 0, 0, 4800));
    if let Some(plugin) = plugin {
        e.execute("mix.insert", &json!({"track": t.0, "slot": 0, "plugin": "eq_7band"})).unwrap();
        e.session_mut().track_mut(t).unwrap().mixer.inserts[0].as_mut().unwrap().plugin = plugin.to_string();
    }
    e.session().clone()
}

/// The session as a file would hold it, opened again (the audio pool is not part of the file).
fn reopen(s: &Session) -> Engine {
    let mut opened = Session::from_json(&serde_json::to_value(s).unwrap().to_string()).unwrap();
    opened.pool = s.pool.clone();
    Engine::new(opened)
}

fn unknown_plugin(r: &Result<Value, EngineError>) -> bool {
    matches!(r, Err(EngineError::BadParams(_, msg)) if msg.contains("unknown plugin"))
}

#[test]
fn plugin_ids_are_unknown_without_hosts() {
    // A session whose mixer insert names a hosted plugin opens; the insert is an unknown plugin.
    let s = session_with_insert(Some("clap:com.example.gain"));
    let mut e = reopen(&s);
    let t = e.session().tracks[0].id.0;
    assert_eq!(e.session().tracks[0].mixer.inserts[0].as_ref().map(|i| i.plugin.as_str()), Some("clap:com.example.gain"));
    let r = e.execute("mix.insert_param", &json!({"track": t, "slot": 0, "param": "gain", "value": 1.0}));
    assert!(unknown_plugin(&r), "{r:?}");
    let r = e.execute("mix.insert_params", &json!({"track": t, "slot": 0, "params": {}}));
    assert!(unknown_plugin(&r), "{r:?}");

    // The hosted plugin lists are empty, rescan or not.
    for query in ["engine.clap_plugins", "engine.vst3_plugins"] {
        for params in [json!({}), json!({"rescan": true})] {
            assert_eq!(e.execute(query, &params).unwrap(), json!([]), "{query} {params}");
        }
    }

    // Every hosted id format is an unknown plugin, as an insert and as an instrument.
    e.execute("track.new", &json!({"kind": "instrument"})).unwrap();
    let inst = e.session().tracks.last().unwrap().id.0;
    for id in HOSTED_IDS {
        let r = e.execute("mix.insert", &json!({"track": t, "slot": 1, "plugin": id}));
        assert!(unknown_plugin(&r), "mix.insert {id}: {r:?}");
        let r = e.execute("mix.instrument", &json!({"track": inst, "plugin": id}));
        assert!(unknown_plugin(&r), "mix.instrument {id}: {r:?}");
    }

    // The built-in plugins stay.
    let built_in = e.execute("engine.plugins", &json!({})).unwrap();
    assert!(built_in.as_array().is_some_and(|l| !l.is_empty()), "{built_in}");
    e.execute("mix.insert", &json!({"track": t, "slot": 1, "plugin": "eq_7band"})).unwrap();
}

#[test]
fn a_session_naming_hosted_plugins_renders_without_them() {
    let dry = soundcraft_mix::render_range(&session_with_insert(None), Range::new(0, 4800), 512);
    assert!(dry[0][2000].abs() > 0.01, "the dry signal must be audible");
    for id in HOSTED_IDS {
        let e = reopen(&session_with_insert(Some(id)));
        let wet = soundcraft_mix::render_range(e.session(), Range::new(0, 4800), 512);
        assert_eq!(wet, dry, "{id}: an insert no host can load passes the signal through unchanged");
    }
}
