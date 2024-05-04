# Road Intersection Simulation Project

## Objectives

The primary goal of this project is to address the traffic congestion problem in a simulated environment representing a capital city's intersection. The project involves creating a traffic control strategy and visualizing it through a simulation using the SDL2 library.

## Instructions

### Environment and Rules

#### 1. Roads
- Construct two roads intersecting each other, each with one lane per direction.
- Vehicles can turn left, right, or continue straight at the intersection.

#### 2. Traffic Lights
- Install traffic lights at each lane entering the intersection, with only red and green signals.
- Implement an algorithm to manage these lights, ensuring minimal traffic congestion and preventing vehicle collisions.

#### 3. Vehicles
- Vehicles must be color-coded based on their intended route.
- Vehicles cannot change their selected route once on the road.
- Each vehicle maintains a fixed velocity and a safe following distance.
- Vehicles must stop at red traffic lights and may proceed on green.

### Commands
- Use arrow keys to spawn vehicles from different directions.
- Press 'r' to spawn a vehicle from a random direction.
- Press 'Esc' to end the simulation.
- Ensure vehicles maintain a safe distance to prevent spamming and collisions.

### Example
An example of the road intersection can be viewed [here](#).

### Bonus Features (Optional)
- Implement animations for vehicles and traffic lights using assets from:
  - [limezu](#)
  - [finalbossblue](#)
  - [mobilegamegraphics](#)
  - [spriters-resource](#)

### Notions
For more information on using the SDL2 library, refer to the [SDL2 documentation](https://docs.rs/sdl2/0.34.3/sdl2/).

## Conclusion
This simulation aims to provide a practical solution to traffic management at a busy city intersection, enhancing traffic flow and safety.
