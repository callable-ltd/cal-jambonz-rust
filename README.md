# cal-jambonz-rust 🚀

A Rust library for building applications that interact with the [Jambonz](https://jambonz.org/) communications platform.

> 📱 **Heads up!** For implementing a complete Jambonz webhook server with Actix Web integration, check out our awesome companion project [jambonz-actix-web](https://github.com/callable-ltd/cal-jambonz-actix-web-rust) - it's perfect for handling those webhooks like a pro!

## Overview 🌟

This library provides a Rust implementation for creating and managing Jambonz applications. Jambonz is an open-source CPaaS (Communications Platform as a Service) that allows developers to build voice and messaging applications using a simple API. This library allows you to create Jambonz applications using type-safe Rust code, abstracting away the complexities of the underlying JSON webhook API.

## Features ✨

- 🛡️ Type-safe implementation of Jambonz verbs and actions
- 🔄 Simplified request/response handling for Jambonz webhooks
- 🎯 Comprehensive support for Jambonz's feature set, including:
    - 📞 Call control (answer, hangup)
    - 🎵 Media operations (play, pause, record)
    - 🗣️ Speech recognition and text-to-speech
    - 👥 Conference management
    - 🔀 Call transfer and SIP integration
    - 🔢 Gather DTMF input
    - And more...

## Requirements ✅

* Rust 1.70+
* Actix Web ecosystem
* Features from: `actix-web`, `actix-ws`, `uuid`, `serde`, `futures`, etc.

## Installation 📦

Add this to your `Cargo.toml`:

```toml
[dependencies]
cal-jambonz-rust = "0.1.0"
```

## Quick Start 🏃‍♂️

Here's a simple example of using the library to create a basic IVR (Interactive Voice Response) application:

```rust
use cal_jambonz_rust::{JambonzResponse, verbs::*};
use cal_jambonz_rust::verbs::synthesizer::Synthesizer;

// Handle an incoming call webhook from Jambonz
fn handle_call_webhook(request_json: &str) {
    // Parse the incoming webhook data
    // (Implementation would depend on your web framework)
    
    // Build a response with Jambonz verbs
    let mut say = Say::new("Welcome to our service!".to_string());
    say.synthesize(Some(Synthesizer::default().voice("female").language("en-US")));
    
    let response = JambonzResponse::new()
        .add_verb(say);
            
    // Print the JSON response
    println!("{}", response.to_json().unwrap());
    
    // The JSON response would look something like:
    // {
    //   "verb": "say",
    //   "text": "Welcome to our service!",
    //   "synthesizer": {
    //     "voice": "female",
    //     "language": "en-US"
    //   },
    //   "loop": 1,
    //   "earlyMedia": false
    // }
}
```

## Supported Jambonz Verbs 🎭

This library supports all standard Jambonz verbs including:

- 🗣️ `Say` - Synthesize speech
- 🎵 `Play` - Play an audio file
- 🔢 `Gather` - Collect DTMF digits
- 🎙️ `Record` - Record the call
- 📞 `Dial` - Make an outbound call
- 👥 `Conference` - Create or join a conference
- 👋 `Hangup` - End the call
- 🚫 `Sip:decline` - Decline an incoming call
- 🚶 `Leave` - Leave a conference
- 🔄 `Transfer` - Transfer the call
- ⏸️ `Pause` - Pause execution
- 🏷️ `Tag` - Add metadata to call events
- ⚙️ `Config` - Configure call parameters
- 🔄 `Enqueue` - Place call in a queue
- 🔄 `Dequeue` - Remove call from a queue

For detailed documentation on each verb and its options, please refer to the code documentation.

## License 📄

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing 🧱

PRs and issues are welcome. Whether it's bug fixes, new features, or documentation improvements — let's build together.

## Acknowledgments 🙏

- [Jambonz](https://jambonz.org/) & Dave Horton, for providing a kick-ass open-source communications platform

## TODO List 📝

Future enhancements planned for this library:

- 🔄 Implement Jambonz REST API client for account management
- ⚡ Add support for real-time call control
- 📚 Create comprehensive documentation with more examples