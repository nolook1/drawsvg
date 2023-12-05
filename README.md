SVG Drawing Test with Bevy

Why? - Testing the capabilities of drawing & rendering SVG's using bevy, bevy_prototype_lyon and bevy_svg

How to install - With the src folder & cargo.toml file installed and inside a directory 
together, all you need is rust installed on your system, open the terminal/ console in 
the directory the src folder and cargo.toml file are in, then use the command "cargo update", 
and next use the command "cargo run" to build and run the application.

Usage - While clicking the left mouse button you can draw a svg with your mouse cursor, 
while drawing the path drawn will be a white line, once the left mouse button is released 
the SVG is drawn on the screen as a black line so you can see when it changes, and the 
SVG is saved into the main folder then into assets/svg/

Controls - V-SYNC toggle - V
           World Movement - WASD
           Draw SVG - Left mouse button
           FPS counter toggle - F12

Known bugs/ Issues - When drawing sometimes if you release the left mouse button the 
SVG will not save & display the SVG properly, it will stay in the drawing mode until 
clicking the left mouse button somewhere on the screen again, not sure why this is, 
you will know this occured if you released left mouse button and the line drawn is still white

Contributing - If you have any ideas/ better ways of doing things please reach out 
and discuss it with me, I am new to bevy and rust in general and would like feedback 
on the code to be honest, thank you for taking the time to read & try the application out!

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
