# Dies-Simulator-RoboCup
<div align = "center">
<pre>
██████╗ ██╗███████╗███████╗    ███████╗██╗███╗   ███╗██╗   ██╗██╗      █████╗ ████████╗ ██████╗ ██████╗ 
██╔══██╗██║██╔════╝██╔════╝    ██╔════╝██║████╗ ████║██║   ██║██║     ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗
██║  ██║██║█████╗  ███████╗    ███████╗██║██╔████╔██║██║   ██║██║     ███████║   ██║   ██║   ██║██████╔╝
██║  ██║██║██╔══╝  ╚════██║    ╚════██║██║██║╚██╔╝██║██║   ██║██║     ██╔══██║   ██║   ██║   ██║██╔══██╗
██████╔╝██║███████╗███████║    ███████║██║██║ ╚═╝ ██║╚██████╔╝███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║
╚═════╝ ╚═╝╚══════╝╚══════╝    ╚══════╝╚═╝╚═╝     ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝
 -------------------------------------------------------------------------------------------------------
Integrated RoboCup 3D simulation environment for testing - in Rust 
By Teodor Neagoe, from team Delft Mercurians
</pre>
</div>

## Goals

### Level 1

- can simulate robot and a ball on a field in 3D
- basic visualization
- can receive commands to control robot from framework

### Level 2

- test harness for unit testing behaviors
    - define conditions
    - define target
    - run simulation with the framework and test

### Level 3

- full match simulation
    - integrate with an autoref

## TODO

- shooter: push the ball forward (plane collider???)
- dribbler: (spinning cylinder that pulls the ball to the circle) ~ apply force when in a defined area
- velocity
    - controller: (target_velocity, actual_velocity) → velocity (fb, )
        - pid controller
    - bgui
- when speed is too high, box goes out of wall
- terrain lines
- robot colors
- Maybe parameters panel, with https://github.com/mvlabat/bevy_egui

## Done (last → first)

- move the robot relative to it’s orientation
- spin robot
- make function to generate box
- add walls
- add ball friction - rolling friction
    - there is no rolling friction, so I added dampling
- move robot independently
- move robot by forces
- move robot
- Graphics: Adding mashes
- add a robot
- add ball
- add terrain
- Make camera movable: bevy_panorbit_camera
    - declare dependency: bevy_panorbit_camera = "0.8.0”
    - import it: use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
    - add to setup_graphincs, in commands.spawn
        - PanOrbitCamera::default()),
- updated default code that used deprecated functions with up to date functions - 5 min
- installation - 1 min
    - cargo init
    - cargo add bevy_rapier3d
    - cargo add bevy
- found 3d game engine to render the info from the physics engine - 5 min
    - Bevy
- found best 3d physics engine - 15 min
    - Rapier
    - areWeGameYet
    - Np Physics
    - Salva

## Ideas

- Headless mode - run physics and get results back without rendering anything
- Browser rendering - view live 3D visualization in a browser window
    - https://bevy-cheatbook.github.io/platforms/wasm.html ?
- Stochastic simulation
    - add random noise to simulation to account for sim2real
    - monte carlo simulation?
    - needs looking into
- Test harness - helper APIs for writing SITL tests
    
    Something like
    

## Used commands

- cargo init
- cargo add bevy_rapier3d
- cargo add bevy