# Bevy Game Development Mastery Curriculum

## Chapter 1: Mental Models & Architecture Patterns

### Core ECS Philosophy
- **Composition Over Inheritance**: Entities gain behavior through component combinations, not class hierarchies
- **Data-Oriented Design**: Components are POD (Plain Old Data) structs; systems contain all logic
- **Archetypal Storage**: Entities with identical component sets are stored contiguously in memory
- **Query-Driven Architecture**: Systems declare data dependencies through type signatures
- **Automatic Parallelization**: Non-conflicting systems run concurrently without explicit threading

### Performance Mental Models
- **Frame Budget**: 16.67ms for 60 FPS - every microsecond counts
- **Cache Coherency**: Components accessed together should be queried together
- **System Ordering**: Minimize sync points between parallel system groups
- **Change Detection Cost**: `Changed<T>` filters have overhead - use strategically
- **Entity Spawning**: Batch spawning is cheaper than individual operations

### Rust + Bevy Idioms
- **Newtype Pattern**: Wrap primitives (`struct PlayerId(u32)`) for type safety
- **Marker Components**: Zero-sized types for entity categorization
- **Resource vs Component**: Singletons are Resources, entity data are Components
- **Bundle Composition**: Group commonly-spawned components for ergonomics
- **System Piping**: Chain systems with `.pipe()` for data transformation

## Chapter 2: Development Environment Mastery

### Critical Configuration Knowledge
- **Debug Performance**: Unoptimized builds run 100x slower - always use `opt-level = 1`
- **Dynamic Linking**: Reduces incremental build time from minutes to seconds
- **Linker Selection**: LLD (Windows/Linux) or zld (macOS) for fastest linking
- **Feature Flags**: `bevy/dynamic_linking`, `bevy/file_watcher`, `bevy/trace`
- **Profile Hierarchy**: dev → release → distribution (increasing optimization)

### Workflow Optimization Patterns
- **Hot Reload Assets**: File watcher + asset preprocessing for instant feedback
- **Inspector Integration**: Runtime component editing accelerates iteration
- **Conditional Compilation**: `#[cfg(debug_assertions)]` for dev-only features
- **Workspace Structure**: Separate game logic from asset pipeline tools
- **CI/CD Templates**: GitHub Actions for cross-platform builds from day one

## Chapter 3: ECS Pattern Library

### Component Design Patterns
```rust
// Newtype for type safety
#[derive(Component)]
struct Health(f32);

// Marker for querying
#[derive(Component)]
struct Player;

// Complex state with methods
#[derive(Component)]
struct Velocity {
    linear: Vec3,
    angular: Vec3,
}

// Relationship components (new in 0.16)
#[derive(Component)]
struct Parent(Entity);
```

### System Architecture Patterns
- **Single Responsibility**: Each system does one thing well
- **Event Broadcasting**: Decouple systems through events
- **State Machines**: Use Bevy States for game flow control
- **System Sets**: Group related systems for ordering
- **Run Conditions**: Skip systems when unnecessary

### Query Patterns & Anti-patterns
```rust
// GOOD: Specific queries
Query<(&Transform, &Velocity), With<Player>>

// BAD: Over-broad queries
Query<&Transform>  // Gets ALL entities with Transform

// GOOD: Change detection
Query<&Health, Changed<Health>>

// GOOD: Optional components
Query<(&Transform, Option<&Velocity>)>
```

### Entity Relationship Patterns (0.16+)
- **Parent-Child**: Automatic transform propagation
- **Inventory Systems**: Items linked to owners
- **Quest Dependencies**: Bidirectional requirement tracking
- **Team Membership**: Many-to-many relationships
- **Damage Attribution**: Track damage sources

## Chapter 4: Asset Pipeline Mastery

### Asset Loading Strategies
- **Eager Loading**: Load during startup for predictable availability
- **Lazy Loading**: On-demand for large worlds
- **Asset Collections**: Group related assets with bevy_asset_loader
- **Handle Patterns**: Strong vs Weak handles for memory management
- **Preprocessing**: Optimize assets at build time, not runtime

### Hot Reloading Workflows
- **File Watcher**: Automatic detection of asset changes
- **Asset Events**: React to reload completion
- **Shader Iteration**: Live shader editing without restart
- **Configuration Files**: RON format for game data
- **Development Assets**: Placeholder assets for rapid prototyping

### Memory Management Patterns
- **Handle Lifecycle**: Reference counting prevents premature unloading
- **Asset Dependencies**: Automatic loading of referenced assets
- **Texture Atlases**: Reduce draw calls through batching
- **Audio Pools**: Reuse audio sources for performance
- **Model Instancing**: Share mesh data across entities

## Chapter 5: Rendering Architecture

### GPU-Driven Rendering (0.16)
- **Indirect Drawing**: GPU decides what to render
- **Frustum Culling**: Automatic visibility determination
- **Mesh Optimization**: Automatic LOD and batching
- **Draw Call Reduction**: 3x performance improvement
- **Virtual Geometry**: Massive mesh support

### Shader Development Patterns
- **Material System**: Custom shaders via Material trait
- **Shader Preprocessing**: Conditional compilation
- **Uniform Binding**: Efficient parameter passing
- **Compute Shaders**: GPU-accelerated calculations
- **Post-Processing**: Screen-space effects pipeline

### Rendering Performance Patterns
- **Sprite Batching**: Automatic for same texture/material
- **Instanced Rendering**: Thousands of similar objects
- **Texture Arrays**: Reduce texture switching
- **Z-Order Management**: Minimize overdraw
- **Render Layers**: Selective rendering for cameras

## Chapter 6: Input & Interaction Systems

### Input Architecture Principles
- **Action Mapping**: Separate logical actions from physical inputs
- **Input Buffering**: Handle inputs between frames
- **Context Sensitivity**: Different input maps per game state
- **Accessibility**: Remappable controls by default
- **Cross-Platform**: Unified handling across devices

### Advanced Input Patterns
```rust
// Action-based input
enum PlayerAction {
    Move(Vec2),
    Jump,
    Attack,
}

// Context-aware handling
fn handle_input(
    state: Res<State<GameState>>,
    input: Res<ActionState<PlayerAction>>
) {
    match state.get() {
        GameState::Playing => process_gameplay_input(input),
        GameState::Menu => process_menu_input(input),
        _ => {}
    }
}
```

### Focus Management (0.16)
- **UI Focus**: Keyboard navigation for accessibility
- **Input Consumption**: Prevent input bleeding through UI
- **Modal Systems**: Exclusive input contexts
- **Gesture Recognition**: Touch and mouse patterns
- **Gamepad Rumble**: Haptic feedback integration

## Chapter 7: Physics Integration Patterns

### Rapier Integration Principles
- **Fixed Timestep**: Deterministic simulation
- **Collision Layers**: Efficient broad-phase filtering
- **Continuous Collision**: Prevent tunneling
- **Sensor Colliders**: Trigger volumes
- **Joint Systems**: Complex mechanical connections

### Physics-Game Logic Separation
```rust
// Physics components
#[derive(Component)]
struct PhysicsBody(RigidBodyHandle);

// Game components  
#[derive(Component)]
struct CharacterController {
    jump_force: f32,
    move_speed: f32,
}

// Bridge system
fn apply_character_physics(
    mut physics: Query<(&mut Velocity, &CharacterController)>,
    input: Res<InputState>,
) {
    // Convert game logic to physics forces
}
```

### Collision Handling Patterns
- **Event Buffering**: Process collisions after physics step
- **Damage Calculation**: Separate from collision detection
- **Trigger Volumes**: Area-based effects
- **Raycast Patterns**: Line-of-sight and selection
- **Physics Debugging**: Visual representation of colliders

## Chapter 8: State Management Architecture

### State Machine Patterns
- **Game States**: Menu → Loading → Playing → GameOver
- **Nested States**: Playing { Exploring, Combat, Dialogue }
- **State Transitions**: Enter/Exit handlers
- **State Persistence**: Save/Load compatibility
- **Parallel States**: Multiple active state machines

### StateScoped Entities (0.16)
```rust
// Automatic cleanup on state exit
commands.spawn((
    StateScoped(GameState::Playing),
    Name::new("Player"),
    // Components...
));
```

### Save System Architecture
- **Component Serialization**: Selective saving
- **Entity Remapping**: Handle ID changes
- **Version Migration**: Forward compatibility
- **Compression**: Efficient storage
- **Cloud Sync**: Platform integration

## Chapter 9: AI & Behavior Patterns

### Behavior Architecture
- **Behavior Trees**: Modular AI decision making
- **State Machines**: Simple reactive behaviors
- **Utility AI**: Dynamic priority systems
- **Goal-Oriented Planning**: Complex strategies
- **Flocking/Swarming**: Emergent group behaviors

### Performance Considerations
- **AI Budgets**: Limit calculations per frame
- **LOD Systems**: Simpler AI for distant entities
- **Batch Processing**: Update groups together
- **Spatial Partitioning**: Efficient neighbor queries
- **Async AI**: Background processing for complex decisions

## Chapter 10: Audio Architecture

### Spatial Audio Patterns
- **3D Positioning**: Automatic panning/volume
- **Audio Sources**: Pooled for performance
- **Dynamic Music**: Layered adaptive soundtracks
- **Audio Events**: Decouple triggers from playback
- **Compression**: Streaming vs preloaded

### Audio State Management
```rust
#[derive(Resource)]
struct AudioState {
    music_volume: f32,
    sfx_volume: f32,
    current_track: Handle<AudioSource>,
}

fn dynamic_music_system(
    game_state: Res<State<GameState>>,
    mut audio: ResMut<AudioState>,
) {
    // Transition music based on game state
}
```

## Chapter 11: Networking Patterns (Advanced)

### Client-Server Architecture
- **Deterministic Simulation**: Lockstep or rollback
- **State Synchronization**: Delta compression
- **Input Prediction**: Client-side responsiveness
- **Lag Compensation**: Server reconciliation
- **Authority Models**: Server authoritative vs peer-to-peer

### Bevy Networking Approaches
- **bevy_replicon**: High-level replication
- **bevy_renet**: Low-level networking
- **GGRS**: Rollback netcode for fighting games
- **Custom Solutions**: Protocol-specific needs
- **WebRTC**: Browser-based multiplayer

## Chapter 12: Performance Optimization Mastery

### Profiling & Metrics
- **Tracy Integration**: Frame profiling
- **System Timings**: Identify bottlenecks
- **Memory Profiling**: Allocation tracking
- **GPU Profiling**: Render pipeline analysis
- **Custom Metrics**: Game-specific measurements

### Optimization Strategies
- **System Consolidation**: Reduce overhead
- **Query Caching**: Reuse expensive queries
- **Spatial Indexing**: Accelerate lookups
- **Object Pooling**: Reduce allocations
- **Async Asset Loading**: Prevent frame drops

### Platform-Specific Optimization
- **WASM**: Minimize binary size
- **Mobile**: Battery and thermal management
- **Console**: Fixed hardware optimization
- **Desktop**: Scalability options
- **VR**: Consistent frame timing

## Chapter 13: Plugin Development

### Plugin Architecture Principles
- **Minimal Dependencies**: Optional features
- **Version Compatibility**: Support multiple Bevy versions
- **Configuration**: Runtime and compile-time options
- **Documentation**: Examples and API docs
- **Testing**: Unit and integration tests

### Community Standards
```rust
pub struct MyPlugin {
    pub config: MyPluginConfig,
}

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_system.run_if(my_condition))
           .init_resource::<MyResource>()
           .add_event::<MyEvent>();
    }
}
```

### Distribution Patterns
- **Cargo Features**: Optional functionality
- **License Choice**: Dual MIT/Apache-2.0
- **Version Policy**: Semantic versioning
- **CI/CD**: Automated testing and publishing
- **Documentation**: docs.rs integration

## Chapter 14: Advanced Rendering Techniques

### Custom Render Pipelines
- **Render Graphs**: Custom rendering flows
- **Multiple Passes**: Complex visual effects
- **Compute Integration**: GPU calculations
- **Custom Vertex Formats**: Specialized data
- **Instancing Strategies**: Massive object counts

### Shader Mastery
- **WGSL Patterns**: Bevy's shader language
- **Shader Variants**: Conditional compilation
- **Material Properties**: Dynamic parameters
- **Screen-Space Effects**: Post-processing
- **Procedural Generation**: GPU-based content

## Chapter 15: Production Patterns

### Project Organization
- **Modular Architecture**: Feature-based modules
- **Asset Organization**: Logical directory structure
- **Code Generation**: Derive macros and build scripts
- **Testing Strategy**: Unit, integration, and playtesting
- **Documentation**: Code, design, and player-facing

### Release Engineering
- **Build Automation**: Cross-platform CI/CD
- **Asset Optimization**: Compression and bundling
- **Performance Budgets**: Target specifications
- **Telemetry**: Anonymous usage statistics
- **Update Systems**: Patching and DLC

### Common Production Pitfalls
- **Entity ID Persistence**: Never store Entity directly
- **System Order Bugs**: Explicit dependencies
- **Asset Loading Race**: Proper state management
- **Memory Leaks**: Unused component cleanup
- **Performance Regression**: Continuous profiling

## Mastery Milestones

### Beginner → Intermediate
- Understand ECS philosophy and patterns
- Build complete 2D games with physics
- Implement save/load systems
- Create reusable game systems
- Profile and optimize performance

### Intermediate → Advanced  
- Develop custom rendering techniques
- Create editor tools and plugins
- Implement networking solutions
- Optimize for multiple platforms
- Contribute to Bevy ecosystem

### Advanced → Expert
- Architect large-scale games
- Develop engine modifications
- Create innovative rendering techniques
- Mentor other developers
- Shape Bevy's future direction