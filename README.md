# ge-flash-briefing

First, we'd like to have Rust be running its latest and greatest. If you don't have Rust installed, please do so. If you do have it, run:
`rustup update`

This will update rust, which is always good! Probably.

Then, we'd like to build our WebUi. You'll need NodeJS and NPM installed. They are pretty easy to get too, so if you don't have em, grab em! It isn't very complicated, but the tools we used to make are very complicated. Go ahead and run:

```
cd webui;
npm install;
npm run-scripts build;
```
This gets us into our webui's directory, installs the dependencies for our build tools, then does that good ole build!

Now we can run our code! To do that, just go ahead and run:
`cargo run`

If you want to run the production code, do:
`cargo run --release`

The production code is a better optimized version that takes significantly longer to compile.
