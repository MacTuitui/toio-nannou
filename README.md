# toio-nannou
Example of using nannou to control toio core cubes using the OSC bridge `toio-osc`

## What?
This is an example of using nannou as a way to control toio core cubes.
The idea is that managing bluetooth connections is painful, but using OSC
not that much.

## How?
You need to run the toio-osc bridge application [here](https://github.com/MacTuitui/toio-osc) 
to connect to the cubes and get things running.
Then:

* cargo run --release

And you should see the cubes on your screen, and they'll follow a hexagonal pattern around.

See the code for more ideas.
