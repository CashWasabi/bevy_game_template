# Kaginawa
A 2D Side Scroller with grappling hooks as mechanic.

# This source codes structure is heavily inspired by:
- [bevy_game_template](https://github.com/NiklasEi/bevy_game_template/tree/main)
- [bevy_ball_game](https://github.com/frederickjjoubert/bevy-ball-game/tree/Episode-10)

# What does this template give you?
* small example ["game"](https://niklasei.github.io/bevy_game_template/) (*warning: biased; e.g., split into a lot of plugins and using `bevy_kira_audio` for sound*)
* easy setup for running the web build using [trunk] (`trunk serve`)
* run the native version with `cargo run`
* workflow for GitHub actions creating releases for Windows, Linux, macOS, and Web (Wasm) ready for distribution
    * push a tag in the form of `v[0-9]+.[0-9]+.[0-9]+*` (e.g. `v1.1.42`) to trigger the flow

# How to use this template?
 1. Click "Use this template" on the repository's page
 2. Look for `ToDo` to use your own game name everywhere
 3. [Update the icons as described below](#updating-the-icons)
 4. Start coding :tada:
    * Start the native app: `cargo run`
    * Start the native app dev: `cargo run dev`
    * Start the web build: `trunk serve`
       * requires [trunk]: `cargo install --locked trunk`
       * requires `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
       * this will serve your app on `8080` and automatically rebuild + reload it after code changes

You should keep the `credits` directory up to date. The release workflow automatically includes the directory in every build.

### Updating the icons
 1. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `build/macos` directory) - _Warning: sadly this seems to require a mac..._
 2. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
    * You can create an `.ico` file for windows by following these steps:
       1. Open `macos/AppIcon.iconset/icon_256x256.png` in [Gimp](https://www.gimp.org/downloads/)
       2. Select the `File > Export As` menu item.
       3. Change the file extension to `.ico` (or click `Select File Type (By Extension)` and select `Microsoft Windows Icon`)
       4. Save as `build/windows/icon.ico`

### Deploy web build to GitHub pages
 1. Trigger the `deploy-github-page` workflow
 2. Activate [GitHub pages](https://pages.github.com/) for your repository
     1. Source from the `gh-pages` branch (created by the just executed action)
 3. After a few minutes your game is live at `http://username.github.io/repository`

To deploy newer versions, just run the `deploy-github-page` workflow again.

=======
 1. Replace `build/windows/icon.ico` (used for windows executable and as favicon for the web-builds)
 2. Replace `build/macos/icon_1024x1024.png` with a `1024` times `1024` pixel png icon and run `create_icns.sh` (make sure to run the script inside the `macos` directory) - _Warning: sadly this seems to require a mac..._

### Deploy web build to GitHub pages
 1. Activate [GitHub pages](https://pages.github.com/) for your repository
    1. Source from the `gh-pages` branch
 2. Trigger the `deploy-github-page` workflow
 3. After a few minutes your game is live at `http://username.github.io/repository`
