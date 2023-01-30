# How Is Ruzzle Created

**想讀中文版[請點這](https://github.com/Rhadow/ruzzle/blob/master/how_is_ruzzle_created_zh_TW.md)**

In 2018, I played Zelda's latest game *Breathe of the Wild* and was fascinated by the world and all the chemistry interactions in the game. As a programmer, the idea of creating a game myself has arised.
People usually thought about using a game engine like Unity when creating games. Using game engine has several advantages: physics, animations, rendering and all the stuff you need to make a game is properly modularized and ready to use. Developers can create a prototype and focus on the gameplay rapidly. The downside is you are missing the fun to implement those logic by yourself.
I have never developed a game from scratch before so I thought this is a good time to try it myself.

## Rust & WebAssembly

My day job is mostly focusing on frontend development, so when thinking on what platform should my first game to be on, web is the first that comes to my mind. I've seen awesome 3D game demos posted online based on **webassembly** and heard of it several times in the past. After reading some articles, I decided to use webassembly in the project mainly due to its performance and its future trend. If you haven't heard about webassembly, [here](https://hacks.mozilla.org/2017/02/a-cartoon-intro-to-webassembly/) is a quick introduction. However, webaseembly is not something you can write directly, it is often compiled from another programming languages like C++, Rust, Go, etc... After doing more research, I decided to adopt Rust. Here are the reasons:

1. The toolchain for compiling Rust to webassembly is relatively complete, there are a lot of tutorials out in the internet
2. Rust seems to be a good language to do game development. Some reasons are mentioned [here](http://arewegameyet.com/)
3. It's fun to learn a language that is [most loved in 2018](https://insights.stackoverflow.com/survey/2018/)

Here is a quick summary on the tools that make Rust, Webassembly and Javascript work together in this project:

### wasm-pack
wasm-pack seems to be the most popular tool for building and compiling Rust to WebAssembly that can interoperate with JavaScript. It can also publish Rust-generated WebAssembly to the npm registry for other people to use with their own project.

### wasm-bindgen
This is probably the most important tool for this project. It creates the glue code between Rust and Javascript allowing them to communicate with each other. In simpler word, Rust can call custom Javascript functions and Javascript can call custom functions written in Rust because of wasm-bindgen is doing its magic behind. Anyone interested can take a look at this [blog](https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/).

### js-sys
js-sys brings Javascript global types and methods into Rust world. An actual use case in my project is dealing with `random` method. The rand crate from Rust does not work in the browser after compiled to webassembly, so my solution is to call `Math.random()` from Javascript in Rust by using `js_sys::Math::random()`.

### web-sys
This is similar to js-sys. It brings all the Web's APIs, such as DOM manipulation, setTimeout etc... to the Rust world. I used it heavily to render game, play audio and retrieving DOM object.

If you follow the [rust-wasm tutorial](https://rustwasm.github.io/book/introduction.html), these tools are included automatically. A small note on web-sys, if there are any features you need and it didn't work in rust, try to update the web-sys features tag in Cargo.toml. If you are not sure how, you can take a look at Cargo.toml in this repository. I have added several features to make web-sys work with canvas, audio etc...

Please correct me if any of the above is wrong. Those are the information I got from their official documents.

### How to Start Dev Environment

Lastly, here are the steps to build and run this project locally, these steps only need to be run for the first time:

1. cd ruzzle
2. wasm-pack build
3. npm init wasm-app www
4. cd www
5. npm install
6. cd ruzzle/pkg
7. npm link
8. cd ruzzle/www
9. npm link ruzzle
10. npm run start

I know there are a lot of steps and its tedious. These steps are documented in more details in the rust-wasm tutorial link above.

If you want a TLDR version, here it is: ruzzle is a rust repository and wasm-pack builds it into webassembly that lives in `ruzzle/pkg`. We also create `ruzzle/www` folder where the website lives in. Npm then links the compiled rust code to the `ruzzle/www` node modules directory so it can be used in the website. Finally, webpack dev server is used to serve the website locally.

When doing development, everytime a change is made in the Rust code, you need to run `wasm-pack build` to compile it in order to see the result. You don't need to restart the server though, webpack will pick it up automatically. This is still not very efficient for development, but the tool chain is evolving and I'm sure running `wasm-pack build` repeatedly will be solved in the near future.

### How to Deploy the Game

1. cd www
2. npm run build
3. push the newly created dist dir content to the gh-pages branch

## Ruzzle Gameplay

Initially, I was planning to make a simple 2D breathe of the wild prototype like Nintendo showed in this [video](https://www.youtube.com/watch?v=ruNLBHDS3yM).
I then realized the scope is too big for a game dev noob's first project. The new plan is to make a puzzle game similar to [this one](http://www.luduminis.com/pascal/) but add chemistry interaction like Breathe of the Wild into it. It is going to be a 2D top down grid base genre with no camera movement. The minimum viable product (MVP) will only include simple objects like cannons, rocks and fire in it. If you are curious about why this game is called Ruzzle, it's just a combined word from Rust and Puzzle.

## Game Architecture

This is the section that gets more technical, I'll try to explain how this game is being put together. Please note, this is my first game project and the architecture is not perfect. In fact, some of my design flaws are already being mentioned in this [awesome article](https://kyren.github.io/2018/09/14/rustconf-talk.html) that I wish I have read it before starting the development. If you are developing a game using Rust, I strongly recommend to read it.

With the introduction get out of the way, lets get started:

### Javascript

The entry point in the Javascript side is `ruzzle/www/index.js`. There is no game logic written in it. It's purpose is to make sure game runs after assets are loaded, bind ui events and rendering. The actual game is written entirely in Rust and its being imported to Javascript as `WebClient`. Javascript only needs to call certain exposed method such as `update` and `render` to get the game started.

### Rust

In Rust side, the entry point is `ruzzle/src/lib.rs`, the game is divided into following modules:

- client
- renderer
- audio
- controller
- utils
- game

### Client

The client corresponds to the platform that the game is going to be running on. Each platform uses different ways to render graphics, play audios and receive user inputs. For example, web uses canvas and HTMLAudioElement to render graphics and play audios. In order to make Ruzzle portable, These logic are separated from the actual game.
Each client will have its own renderer, audio and controller module to deal with platform specific logic. Client also includes an instance of the actual game world and other metadata.

### Renderer

Renderer is used to render the game graphics. There are traits defined in `ruzzle/src/renderer/mod.rs`. With this approach, the rendering logic is being abstracted away from the game. Ruzzle only needs to call something like `draw_objects` and the renderer will handle the rest.

### Audio

Same idea with audio. Traits are defined in `ruzzle/src/audio/mod.rs`

### Controller

Controller is used to map user's input to the game. Initially, I didn't extract controller to a module. The original design was something like "call `handle_player_movement` directly when keydown event happens". Unfortunately, this approach introduce a significant delay to player control.
The solution to this issue is to maintain a map of which keys are presently pressed. The game then reads the controller map for further computation. I'm not sure if this is needed for other platforms, but extracting it to a standalone module is always good for further extension.

### Utils

Utils is the place where general functions reside. Functions like `coordinate_lerp` for animation and `check_collision` for collision detection are put in here. By the way, the collision system in this game uses rectangle area which leetcode has a [problem on it](https://leetcode.com/problems/rectangle-area/). Finally see a leetcode question in action in real projects.

### Game

This is where the main game logic lives and it is further divided into several modules:

- scenes
- assets
- constants
- status_manager
- terrains
- tile
- character
- objects
- level
- world

Let's go over them one by one:

### Scenes

There are currently three scenes in Ruzzle, the entry scene on the initial load, level selection scene and the game scene where the level is being instantiated. I have created traits for scenes so each scene can have its own rendering logic and handlers when input event happens.
To handle scene switching, each scene has an optional `next_scene` attribute. When a scene switching is happening, current scene will set its next scene base on its own logic in its `update` method. The client will then detect that current scene's `next_scene` attribute is not `None` anymore and render the new scene in next frame.
Honestly, I don't think it's a perfect design here. A possible refactor direction is to use the [observer pattern](http://www.gameprogrammingpatterns.com/observer.html).

### Assets

Asset stores information of sprite for a game entity. There are four sprite sheets in this game, when the game wants to draw an entity, it needs to know which sprite sheet to use, what position in the sprite sheet the object is at and what is the sprite size. When an entity status change, for example a tree turning into burning tree, the tree entity needs to update its asset to use a new sprite to show the fire. With asset, code for entities with same behavior but different skins can be reused.

### Constants

All the game constants, nothing special.

### Status Manager

Status manager controls the status of entity. What size it is, where it is at on the map, which direction it is facing etc... Every entity (characters, objects and terrains) in this game has it. There is a design flaw here, entities like terrain doesn't need to know if it's walking or dying but theses information are included. A possible solution is to implement ECS (Entity Component System) instead of using OO like approach (Rust doesn't support inheritance directly).

The logic for moving characters and items smoothly also resides here. As a grid based game, entities like characters use `Position` (row, column) to locate themselves. One problem with position is the entity snaps to the target position directly when moving, there is no smooth animation of moving from point A to point B. The solution is to add `Coordinate` (actual x and y relative to canvas) for entities. It allows the entity to locate between tiles when moving. Position and Coordinate can be easily converted into one another using methods in status manager.

### Terrains

So far in Ruzzle, there are only two terrains: `Land` and `Hole`. Each terrain will have its own update logic for example falling is handled in the `update` of `hole.rs`.

### Tile

The map in the game is created by an array of tiles called `tile_map`. Each tile contains a terrain in it. It seems like an extra wrapper for terrains for now, but the plan is to add other attributes into tiles such as special effect in the future.

### Character

This is where the player character code resides. The traits for characters are also defined here.

### Objects

Objects are all the other entities except characters on the map. They have an extra `attribute_manager` to control their behavior. For example, `temperature` shows how hot this object is, `is_burnable` indicates whether the entity can be burned. This `attribute_manager` serves similar purpose as `status_manager`, what makes them different is attributes in `attribute_manager` are only limited to objects. If for example, characters need `temperature` in the future, the `temperature` attribute needs to be moved to `status_manager`. I personally think this is a design flaw and again it should be solved by ECS.

### Level

Every levels and level editor is stored in this module. Each level is composed by three elements: terrains, objects and player initial position. Terrains and objects are array of strings. Each string is a key that maps to its correspond entity for example 'T' maps to tree. The full documentation of this key/entity map is documented in `ruzzle/src/game/level/level_manager.rs`. Player initial position is a row/column tuple. One extra note, the spawn point object is automatically added to player's initial position.

If you are wondering why spawn point is needed, The answer is to prevent user move objects to the player's respawn point. since when player dies, he will be respawned at spawn point and having an object there doesn't make any sense.

### World

World is everything above combined together. A world has a tile map, list of objects and list of characters. It also has state indicating which level the user is currently playing and whether it is completed or not. World also provides setters and getters for different entities where most game logic will use.

## Game Art

Whew, with the technical part done, let's let our left brain to rest a bit and put the right brain to work. Speaking of the game art, I was originally going to use some of the resources from [opengameart.org](https://opengameart.org/) since I haven't been drawing anything except flow chart and [trees](https://en.wikipedia.org/wiki/Tree_(data_structure)) after high school, not to mention creating music. However, learning something new is always the main goal of doing these side projects so I decided to draw the assets myself. If you are asking me why not also compose game music yourself, I would say I thought about it and I have found some cool channel on youtube such as [this one](https://www.youtube.com/channel/UCeZLO2VgbZHeDcongKzzfOw) to learn how to compose music. Then I realized drawing has already taken a lot of time than I originally planned. In fact, half of the development time of this game is drawing sprites, so maybe I'll try music in the next project.

The program I used to draw sprite is [aseprite](https://www.aseprite.org/), it is recommended by most artist online. Initially, I thought drawing these low resolution characters are easy but it turns out I'm totally wrong. Since there aren't many pixels you can utilize in pixel art, everything needs to be calculated to make the image look natural, some details needs to be simplified into one pixel or even ignored. Also, there are a lot of techniques to learn such as different types of shadows, in what angle the user is perceiving etc... To draw a decent image, it requires tons of practices and observations in our daily life.

Here are some tutorials if you are interested in creating pixel art:

- [https://medium.com/@davidbyttow/a-quick-and-dirty-guide-to-creating-pixel-art-d3d43d4bf421](https://medium.com/@davidbyttow/a-quick-and-dirty-guide-to-creating-pixel-art-d3d43d4bf421)
- [http://blog.studiominiboss.com/pixelart](http://blog.studiominiboss.com/pixelart)
- [https://medium.com/pixel-grimoire](https://medium.com/pixel-grimoire)

## Conclusion

Developing a game with Rust is a weird experience, I feel safe when making code changes as the mighty compiler will always do a final check before the program starts running. On the other hand, everytime I see `Cannot borrow as mutable because it is also borrowed as immutable` or `cannot infer an appropriate lifetime for lifetime parameter`, it gives me a lot of headache. In general, programming in Rust will make you a shaper programmer as these errors often leads to subtle bug in other languages. It may take you hours to make those error messages go away at first, but as you get used to it, these errors can be prevented beforehand or fixed sooner.

I always know writing a game from scratch is not an easy task, but after creating the MVP of Ruzzle, I realized it's way harder than I originally thought. Here is my personal list of requirements to make a decent indie game:

1. Good game architecture to do enhancements and add new feature to it
2. Good art and music to make player engaged
3. Good level design to challenge the player without losing their patience

I'm sure there are more than what I listed. At the time of this writing, Ruzzle doesn't even accomplish any of the above points, there are a lot of features missing. I'm planning on enhancing Ruzzle by adding features like save/load mechanism, more elements to interact with, better user control, better support for mobile device, etc....

As a toy project, I did learn quite a lot from it. Rust, webassembly toolchain, game development, pixel art are all new to me and the result is not too bad (at least it's playable). Lastly, thank you for reading this post and if you are interested in enhancing Ruzzle, PR is always welcome!
