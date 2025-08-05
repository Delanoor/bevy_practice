# Bevy Practical Learning Path: From Zero to Game Developer

## Lesson 1: Making Something Appear on Screen

### The Goal
"I want to see something on screen that I created."

### What You Need to Know First

**The Mental Model**: In Bevy, you don't "draw" things. You create entities (things that exist) and give them components (properties). Bevy figures out how to draw them.

Think of it like this:
- **Entity** = A thing in your game (player, enemy, wall, UI button)
- **Component** = A property of that thing (position, color, health, sprite)
- **System** = Code that acts on things with certain properties

### Your First Challenge

Create a red square in the center of the screen. But here's the catch - I'm only giving you the structure:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .run();
}

fn setup_game(mut commands: Commands) {
    // First, spawn a camera - without this, you see nothing!
    commands.spawn(Camera2dBundle::default());
    
    // Now spawn your red square
    // Hint: You need a SpriteBundle with:
    // - A Sprite component that has a color (try Color::rgb(1.0, 0.0, 0.0))
    // - A custom_size (try 50x50 pixels)
    // The rest can be ..default()
}
```

### Key Insights to Engrave in Your Brain

**1. Nothing Exists Until You Spawn It**
- No camera? Black screen forever
- Forgot to spawn something? It doesn't exist
- This is different from "draw a rectangle" style programming

**2. Bundles Are Just Convenience**
```rust
// A SpriteBundle is just a pre-made collection of components:
// - Transform (position, rotation, scale)
// - Sprite (color, size, flip, etc.)
// - Visibility (is it visible?)
// - GlobalTransform (calculated world position)
// ... and more

// You could spawn these individually, but bundles are easier
```

**3. The ..default() Pattern**
```rust
// This means "I only care about some fields, use defaults for the rest"
SpriteBundle {
    sprite: Sprite {
        color: Color::BLUE,  // I care about this
        ..default()          // Use defaults for everything else
    },
    ..default()              // Use defaults for transform, visibility, etc.
}
```

### Try These Variations

1. Make a green square instead
2. Make it 200x50 pixels (a rectangle)
3. Make two squares - one red, one blue
4. Position one at coordinates (100, 0) - hint: look at the `transform` field

### The "Aha!" Moment

When you run this and see your square, realize what just happened:
- You never wrote a game loop
- You never cleared the screen
- You never called any draw functions
- You just described what exists, and Bevy handled the rest

**This is the power of ECS** - you describe your game world, not how to render it.

### Real Game Scenario

"The artist just decided all enemies should be purple instead of red"

Traditional engine: Find every place you draw enemies, change the color
Bevy: Change one color value where you spawn enemies. Done.

## Lesson 2: Making Things Move

### The Goal
"I want my square to move across the screen."

### The Setup Challenge

First, modify your previous code:
1. Add a marker component called `MovingSquare` (just an empty struct with `#[derive(Component)]`)
2. Add this component to your square when spawning
3. Create a new system that runs in `Update` (not `Startup`)

### The Movement Challenge

Your system should:
- Find all entities with both `Transform` and `MovingSquare`
- Move them 100 pixels per second to the right
- Use `time.delta_seconds()` to make movement smooth

Structure:
```rust
fn move_squares(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MovingSquare>>
) {
    // Your code here
    // Hint: transform.translation.x += speed * time.delta_seconds()
}
```

### Key Insights to Engrave

**1. Queries Are Your Primary Tool**
```rust
Query<&mut Transform, With<MovingSquare>>
// This reads as: "Give me mutable access to Transform 
// for all entities that also have MovingSquare"
```

**2. Marker Components Are Free**
- They take zero memory
- They're just for categorization
- Use them liberally: `Player`, `Enemy`, `Projectile`, `Collectible`

**3. Frame Independence Is Not Optional**
```rust
// WRONG - Moves different speeds on different computers
transform.translation.x += 5.0;

// RIGHT - Moves same speed everywhere
transform.translation.x += 100.0 * time.delta_seconds();
```

### Experiment Time

1. Make it move up instead of right
2. Make it move diagonally
3. Add a second square that moves at a different speed
4. Make one square that moves in a circle (hint: use `time.elapsed_seconds()` with sin/cos)

### The Power Moment

Spawn 100 squares with the `MovingSquare` component. Watch your one system automatically handle all of them. No loops needed in your code - Bevy handles the iteration.

```rust
// Spawn many squares
for i in 0..100 {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(i as f32 * 10.0, 0.0, 0.0),
            ..default()
        },
        MovingSquare,
    ));
}
```

### Real Game Scenario

"We need different enemy movement patterns"

Instead of different enemy classes, just make different marker components:
- `MovesInStraightLine`
- `MovesInCircle` 
- `FollowsPlayer`
- `PatrolsBetweenPoints`

Each gets its own simple system. Enemies can even have multiple movement components for complex behavior!

## Lesson 3: Responding to Input

### The Goal
"I want to control something with my keyboard."

### The Data Design Challenge

Before writing code, design your components:
1. Create a `Player` component with a `speed` field
2. Think: what other data might a player need? (health? ammo? score?)
3. Keep it simple for now - just speed

### The Input Challenge

Create a system that:
- Reads keyboard input using `Res<ButtonInput<KeyCode>>`
- Finds entities with both `Transform` and `Player`
- Moves based on arrow keys (or WASD)
- Maintains consistent speed when moving diagonally

Structure hint:
```rust
fn player_input(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>
) {
    // Build a direction vector based on input
    // Normalize it so diagonal movement isn't faster
    // Apply movement using player.speed and time.delta_seconds()
}
```

### Key Insights to Engrave

**1. Input is Just Another Resource**
```rust
// Bevy updates this every frame before your systems run
keyboard.pressed(KeyCode::Space)    // Is it held down?
keyboard.just_pressed(KeyCode::Space)  // Was it pressed this frame?
keyboard.just_released(KeyCode::Space) // Was it released this frame?
```

**2. Normalize Diagonal Movement**
```rust
// Problem: pressing right+up moves at 141% speed (diagonal of unit square)
let mut direction = Vec3::ZERO;
if keyboard.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
if keyboard.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }

// Solution: normalize the vector
if direction.length() > 0.0 {
    direction = direction.normalize();
}
```

**3. Component Data Drives Behavior**
```rust
// Don't hardcode values
transform.translation += direction * 200.0 * time.delta_seconds(); // Bad

// Use component data
transform.translation += direction * player.speed * time.delta_seconds(); // Good
```

### Advanced Challenges

1. Add acceleration/deceleration instead of instant movement
2. Add a boost when holding Shift (double speed)
3. Prevent the player from leaving the screen
4. Add rotation - make the square face the direction it's moving

### The Design Pattern

Notice how we're building:
- **Data** (components) is separate from **behavior** (systems)
- Multiple systems can act on the same entity
- Easy to add new features without touching existing code

### Real Game Scenario

"We want to add controller support"

With traditional input handling: Rewrite your input code
With Bevy: Add another system that reads gamepad input, modifies the same `Player` components. Both systems can run!

## Lesson 4: Collision Detection Basics

### The Goal
"I want something to happen when things touch."

### Design First

Before coding, think about what components you need:
1. A `Collider` component with a size
2. Different markers: `Player`, `Enemy`, `Collectible`
3. Maybe a `Health` component?

### The Collision Challenge

Without using a physics engine (we'll learn that later), implement basic AABB collision:

```rust
#[derive(Component)]
struct Collider {
    size: Vec2,
}

fn check_collisions(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    enemy_query: Query<(&Transform, &Collider), With<Enemy>>,
    mut commands: Commands,
) {
    // For each player...
    // For each enemy...
    // Check if their rectangles overlap
    // If they do, do something (remove enemy? damage player?)
}
```

### Key Insights to Engrave

**1. Multiple Queries in One System**
```rust
// You can have different queries for different entity types
// This is more efficient than one giant query with filters
```

**2. Entity Relationships Are Temporary**
```rust
// Don't store references to other entities in components (usually)
// Instead, calculate relationships each frame
// This prevents dangling references when entities are removed
```

**3. Commands Execute After the System**
```rust
// This is safe - commands are queued
for (entity, transform, health) in &query {
    if health.0 <= 0.0 {
        commands.entity(entity).despawn();
    }
}
```

### Experiments

1. Make collectibles that disappear when touched
2. Add a score counter (hint: use a Resource)
3. Make enemies that damage the player
4. Add invincibility frames after taking damage

### The Architecture Insight

Notice how collision detection is separate from:
- Movement (different system)
- Rendering (handled by Bevy)
- Input (different system)
- Health/damage (could be another system)

Each system has one job. This is the Unix philosophy applied to game development.

## Lesson 5: Game States

### The Goal
"I want a menu, gameplay, and game over screen."

### The State Challenge

Implement three states:
1. `MainMenu` - Shows "Press Space to Start"
2. `InGame` - The actual gameplay
3. `GameOver` - Shows "Game Over! Press R to Restart"

```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
```

### Key Insights to Engrave

**1. States Control System Execution**
```rust
// This system only runs in the InGame state
app.add_systems(Update, player_movement.run_if(in_state(GameState::InGame)));
```

**2. State Transitions Are Events**
```rust
// Change state by setting the NextState resource
fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}
```

**3. OnEnter/OnExit for Setup/Cleanup**
```rust
// Spawn game entities when entering InGame
app.add_systems(OnEnter(GameState::InGame), spawn_game_entities);

// Clean up when leaving
app.add_systems(OnExit(GameState::InGame), cleanup_game);
```

### Advanced Challenge

Create a pause state that:
- Stops game logic but keeps rendering
- Shows a pause menu overlay
- Can be entered/exited from InGame only

### Real Game Scenario

"We need to add a tutorial before the main game"

Just add a new state! No need to restructure existing code. Your game systems don't need to know about the tutorial.

## Lesson 6: Events - The Game Communication System

### The Goal
"I want different parts of my game to communicate without tight coupling."

### The Event Challenge

Create an event system for game actions:

```rust
#[derive(Event)]
struct ScoreEvent {
    points: i32,
}

#[derive(Event)]
struct PlayerDamagedEvent {
    damage: f32,
}

#[derive(Event)]
struct EnemyDefeatedEvent {
    enemy_type: EnemyType,
    position: Vec3,
}
```

### Key Insights to Engrave

**1. Events Decouple Systems**
```rust
// Instead of: collision system directly modifying score
// Use: collision system sends event, score system listens

// Collision system
events.send(ScoreEvent { points: 10 });

// Score system (separate!)
for event in score_events.read() {
    score.0 += event.points;
}
```

**2. Events Are Frame-Bounded**
```rust
// Events exist for exactly 2 frames then disappear
// This prevents accumulation and memory leaks
```

**3. Multiple Listeners Are Free**
```rust
// Many systems can listen to the same event:
// - UI system updates score display
// - Audio system plays sound
// - Achievement system checks for milestones
// - Analytics system logs the event
```

### Design Exercise

Create an event flow for:
1. Player collects powerup
2. Powerup sends `PowerupCollected` event
3. Multiple systems respond:
   - Inventory system adds powerup
   - UI system shows notification  
   - Audio system plays sound
   - Stats system tracks collection

### Real Game Scenario

"We need to add achievements"

With events: Just add an achievement system that listens to existing events. No changes to game logic needed!

## The Path Forward

Each lesson builds on the previous one. By Lesson 6, you're not just writing code - you're architecting games. You understand:

- Why ECS exists (performance + flexibility)
- How to structure game data (components)
- How to write game logic (systems)
- How to manage game flow (states)
- How to coordinate systems (events)

This is the foundation. Everything else - physics, audio, networking - builds on these concepts.