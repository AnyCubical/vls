# Verkehrsleitsystem DHBW Verteilte Systeme

This is a Rust project that simulates a simple traffic control system. It consists of several modules that represent different aspects of the system, such as coordinates, traffic areas, and the logic for controlling the traffic.

## Modules

### Coordinates

This module defines the Coordinate struct, which represents a pair of 'x' and 'y' coordinates. It provides methods for creating a new coordinate, getting the 'x' and 'y' values, and implementing the 'Clone', 'Debug', and 'Default' traits.

### Movement Not Possible

This module defines the 'MovementNotPossible' struct, which represents an error that occurs when a movement is not possible. It provides a method for creating a new error message and implements the 'Debug', 'Display', and 'Error' traits.

### Traffic Area

This module defines the 'TrafficArea' struct, which represents a two-dimensional area of traffic nodes. It provides methods for creating a new traffic area, placing and removing clients at specific coordinates, getting the position of a client, checking if a position is free, and clearing the entire area. It also implements the 'Debug', 'Display', and 'Clone' traits.

### Traffic Control Logic

This module defines the 'TrafficControlLogic' struct, which represents the logic for controlling the traffic. It provides methods for starting a new client, moving a client to a specific coordinate, and calculating the distance between two coordinates. It requires a 'TrafficArea' object to function properly.

## Usage

To use this project, you can add it as a dependency in your Rust project's Cargo.toml file:

```rust
[dependencies]
vls-dhbw = "0.1.0"
```

Then, you can import the modules you need in your Rust code:

```rust
use vls_dhbw::coordinates::Coordinate;
use vls_dhbw::traffic_area::TrafficArea;
use vls_dhbw::traffic_control_logic::TrafficControlLogic;
```

## License

This project is licensed under the MIT license. See the LICENSE file for more information.