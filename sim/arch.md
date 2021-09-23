# Architecture for Core Simulation

## Concepts
- Split into deciding and acting phases
- Actions are events 
- Actions can be handled by dispatching lower-level actions
- Deciding phase is read-only 

## Exploration 
**Should actions be strongly typed?**
  - Does an action Trait make sense?
  - What characterizes an action?

**Should agents be able to take more than one action per step?**
  - Need some way to prevent unlimited actions 
  - Don't want to be overly strict
  - Eventually actions could have a `time_cost` and there is a time budget per tick

**How should environment updates be scheduled?**
  - Can everything be an agent? What are consequences of this?
    - Intuitively no. Processes where there is no decision going on shouldn't be modelled as agents
  - Updates could be considered background systems that are triggered without needing agent actions.

**How should decision logic be grouped?**
  - Each agent has one type gets a decision tree. 
    - Components can be reused but there is one root
  - Each agent can belong to multiple types and each has a separate decision tree. 
    - Decoupled logic means greater reuse may be possible and automatic
    - Likely confusing to reason about all the decision logic an agent may be doing
    - Unexpected cross-interactions likely to lead to bugs
    

## Application to Sim

### Agents
Currently only one agent type

**Trader**

Attributes:
- `Cargo` of 1 `Good`
- `Location` on the graph
    - limited to nodes for now, with extension to edges later
- `Money` - how much liquid money they have
    - would be great to add credit as a concept later
    
Actions:
- `Buy` a `Good`
    - requires enough money to cover cost
    - good must be in stock
    - should extend with 'limit' price to guard against large price swings
- `Sell` a `Good`
    - inverse of `Buy` without restrictions
- `Move` from current `location` to new `location`
    - locations must be adjacent
    
### Update Systems

**Clearing House**

Aggregates all `Buy` and `Sell` orders by location and good type, 
computes changes for all entities, 
rejects orders that are invalid (too little money or not enough goods)
adds and subtracts money from entities. 

**Mover**

Validates `Move` actions and executes them.

**State Exporter** 

Sends the current state to the server module 

**Tick Updater**

Increments the simulation tick
