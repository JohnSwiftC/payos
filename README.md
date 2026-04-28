# Payos

> "What in the hell is a modified payphone and why does it need a TUI"

Payos is a Rust-powered TUI running inside a real, heavily modified payphone. It's driven by a Raspberry Pi embedded in the chassis, using the original hardware, keypad, receiver, and even the card sensor, as inputs to control a fully interactive terminal system.

## What It Does

Payos runs on and interfaces with a modified payphone off FB Marketplace. It:

* Reads direct input from hardware: the keypad, card reader, and phone lever
* Has audio working on the phone itself
* Runs a custom TUI which includes games, config menus, calling, etc.
* Launches its own webserver for configuration
* Is a lot of fun and a great piece of furniture

## Why This Exists

My roommates and I love to have fun things around us. Some people might call it garbage, but one man's trash is another man's treasure or something like that. Examples include:

* A super cool bear riding a shark shower curtain (me)
* A super cool shark wall sculpture (it looks like it's going into the wall) (also me)
* Lots of Enron merch (Leo)
* Lots of furniture (Jack)

It only fits that I revive the payphone I started building in high school and bring it to the big WL, Indiana.

## Hardware and IO Interfacing

The payphone is, at its core, a stock coin-op unit with a controller board inside that handles the keypad, lever, and card reader. It does not come out of the box with the ability to talk to a Raspberry Pi. As such, I spent a solid amount of time reversing a lot of the things that I would want to use.

The board on the phone directs every output to a 24-pin header. Out of those 24, a fair amount correspond directly to the matrix used on the keypad. Using the state of the art in electrical tooling (a GPIO prod and an LED), I mapped out which pins I needed to poll to read keypad inputs.

The phone receiver lever followed next, with two corresponding pins which connected a circuit as the lever is pressed down.

Finally, the credit card reader. The reader itself appeared to be very proprietary, that alongside me not really wanting to skim peoples credit cards, led me to find a set of two other pins which just told me if a card was inserted.

With a pin-out drawn and ready to use on the Pi, I cooked up a Python script to probe pins and send keystrokes to the OS, turning all of these buttons into a sort of keyboard. It utilizes a standard matrix polling algorithm to accurately draw keystrokes while ensuring to not allow duplicate keys in the same press. Other hardware probes, such as the lever, just run constantly and provide a stable "on/off" signal.

Audio came after. Any reasonable person would want their payphone to be able to do what it's built for: make calls. The Raspberry Pi I chose had the correct modules for analog audio, so with some cutting and twisting of the cables, I managed to get the speaker and microphone working.

> There was an incredible amount of trial and error at this stage, maybe there's a better way, but I'm not a hardware type of guy...

## Software

I want my payphone to be fun. As such, the software is where that really needs to get done. My initial version which I wrote when I was still in high school was entirely in Python, and acted as a simple CLI program that did fun things but was relatively simple and didn't have much going for it.

The new version is much more interesting. It splits into two pieces: a TUI that runs on the phone itself, written in my current favorite language, Rust, using the Ratatui library for rendering, and a Go webserver that handles configuration. The TUI is where most of the work went, so that's where I'll start.

## Writing The New TUI (Where it gets interesting)

Ratatui is **not** a UI framework, at least if you compare it to the Reacts of today. It's an incredibly strong rendering library that gives you some good patterns and some base components (called widgets), but it does not come out of the box with a full framework.

So, I had the joy of writing my own. Here's how it works:

* A main app is constructed, which is the main holder of state across the application.
* The main render loop is started.
* First, check for and render any interrupt that's currently in the app state.
* Second, render the page currently on the *page stack*, if there are any.
* Third, prop input events to the current active page for handling.
* If the event callback has returned a page, push it to the page stack.
* Repeat the loop until the payphone dies or it crashes.

### The Page Stack and Pages

The main basis of my TUI. The app maintains a stack of *Pages*; the page at the top of the stack is the current page being rendered. A `Page` has three main components behind it:

* The render function
* An event callback
* An on-load function

The render function is how the page is *actually* rendered. It's given access to global app state and the screen buffer so widgets can be rendered there.

The event callback is where pages are given the ability to be dynamic and do interesting things. It is given access to app state and a keyboard event. It returns an optional `PageSignal`. If one is returned with a page, it is added to the stack.

The on-load function is simple. It is called a single time when a page is first rendered and then never again. This was initially added because the setup required to render an image is expensive and requires cloning, so it was vastly more efficient to do it only one time at page load.

> Matter of fact, the TUI would not even run images if this was not added.

These are great and serve as the backbone of the current app, but there are limitations. First, the `Page` type uses normal Rust `fn` pointers. This means that they can't be dynamically created and used like you can with components in React. Second, they cannot be intentionally blocking. Adding a sleep call, for example, in any of these functions breaks the rendering loop.

I first addressed dynamic components in my framework.

### WidgetFn

`WidgetFn`s are simply a boxed dynamic render function. Notably, these are `Box<dyn Fn>`, not the `fn` type, meaning the trait system gives us more flexibility over what these contain and how they can be constructed.

The main benefit and use is in `Page`s where the same element is repeated but needs to have some dynamic state or quality. It's primarily used when rendering grids with buttons. For example, a rich button `WidgetFn` can be created with just a function call and then dynamically rendered by a page using the `Grid` type.

### Interrupts

The final piece of my framework, and arguably just as important as the `Page`. An interrupt has a render function like a `Page`, but also includes a callback that is called *after* the interrupt is rendered.

This allows for complete control over the flow of the app. For example, I use interrupts to spawn pop-ups that last a fixed time, and to spawn a web server that blocks the TUI until it dies.

Finally, interrupts can be used anywhere. They can be added to the app state in a render function, in an event callback, or even on page load. They can do almost anything to app state in their callback, making them not just useful to my framework, but also an escape hatch if I ever need to do something complex.

## It Runs Its Own Webserver

The webserver, written in Go with the incredibly popular Gin package, solves a host of technical issues I experienced in the first version. Most importantly, **configuration**.

Payos maintains an SQLite database which stores configuration for different components of the TUI, like credentials for third-party APIs, lists of names for games, passwords, etc. However, modifying these values on the phone ranges from difficult to impossible, as a user can only make a small subset of inputs on the keypad. My solution? When anyone wants to change config, a user selects an option on the phone and spins up a server on LAN to modify these values.

It's also pretty cool for a payphone in my opinion :)

## Was It Worth It?

A great part of this project is how open it is. I get to build features because I either find them to be fun or interesting, which is something that's rare to find even in personal projects. In many of my own tools, the topic is interesting to me, but the scope of features may be limited due to the specific use-case of the project. I can't (or I guess shouldn't) write a cat rendering feature into rustdllproxy.

But my favorite part is that I get to build alongside my friends. Most of my friends, including my roommates, are not invested into programming or CS. Even among those who are, many are still focused on different niches than I am. It's special to have a project that allows for so much depth but can also be used and appreciated by everyone you know.

I think it's important to remember that programming doesn't always have to be focused towards resume bullets or landing the next big thing. I'm a firm believer that if you build what you enjoy, build what you think is fun, the skills and credibility will come.
