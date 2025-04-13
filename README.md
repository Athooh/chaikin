Here's the updated README with a link to the MIT License instead of including the full text:

# Chaikin's Algorithm Visualization

## Overview

This project implements Chaikin's algorithm for curve smoothing with an interactive visualization. Users can place control points on a canvas and watch as the algorithm generates a smooth curve through step-by-step animation.

## Getting Started

### Prerequisites
- Rust and Cargo installed on your system

### Installation and Running

1. Clone the repository:
```bash
git clone https://learn.zone01kisumu.ke/git/seodhiambo/chaikin.git
cd chaikin
```

2. Run the program:
```bash
cargo run
```

## Features

- Interactive canvas for placing control points with mouse clicks
- Visual representation of Chaikin's algorithm steps
- Animation that cycles through 7 steps of the algorithm
- Clean UI with instructions and step counter
- Input handling for controlling the visualization

## Controls

- **Left-click**: Add control points
- **Enter**: Start animation (requires at least 3 points)
- **C**: Clear all points and reset animation
- **Escape**: 
  - During animation: Return to drawing mode
  - During drawing: Quit the program

## Behavior

- With **1 point**: Only shows the single control point
- With **2 points**: Draws a straight line between them
- With **3+ points**: Animates through 7 steps of Chaikin's algorithm
  - Original points remain visible (red)
  - Current step is shown in green
  - Step counter displays progress (e.g., "Step: 3/7")

## Technical Implementation

The project consists of several modules:

1. **Chaikin Algorithm** (`chaikin.rs`):
   - Implements the core curve smoothing algorithm
   - Generates intermediate steps for animation

2. **Animation System** (`animation.rs`):
   - Manages the animation state and timing
   - Handles step transitions and looping

3. **Input Handling** (`input.rs`):
   - Processes mouse and keyboard input
   - Translates inputs into actions

4. **Rendering** (`renderer.rs`):
   - Draws points, lines, and UI elements
   - Shows instructions and status messages

5. **Main Loop** (`main.rs`):
   - Coordinates all systems
   - Manages the application state

## Requirements Verification

The implementation meets all specified requirements:

- ✔️ Mouse input for placing control points
- ✔️ Visual indicators for control points (red circles)
- ✔️ Animation starts with Enter (requires ≥3 points)
- ✔️ Handles edge cases (1-2 points) correctly
- ✔️ 7-step animation cycle
- ✔️ Escape key to quit
- ✔️ Clear points functionality (C key)
- ✔️ Helpful UI messages

## Bonus Features Implemented

- ✅ Clear screen functionality (C key)
- ✅ Informative message when attempting to start animation with insufficient points
- (Note: Real-time dragging of control points is not implemented)

## Learning Resources

For more information about Chaikin's algorithm:
- [Chaikin's Algorithm Explanation](http://graphics.cs.ucdavis.edu/education/CAGDNotes/Chaikins-Algorithm/Chaikins-Algorithm.html)
- [Reference Video](https://youtu.be/PbB2eKnA2QI)

## License

This project is licensed under the [MIT License](LICENSE).