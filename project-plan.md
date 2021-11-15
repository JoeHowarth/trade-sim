# Publish Trade Sim
**Contents** 
1. Requirements
2. Projects
	a. Design docs
3. Timeline

## Requirements 
**Core**
- Support N goods
- Multi-step look-ahead
- Support integer edge distances

**Analysis** 
- Table w/ actions taken by agents
- Chart money over time per agent
- Chart price of goods over time across cities and across goods 
- Measure efficiency of market
	- Average and standard deviation of price of goods across cities 
- Download full scenario data
- Move backwards and forwards through time 

**Scenario configuration**
- Upload yaml file to frontend
- Form generated from json schema

## Projects

### (A) Cleanup Data Model (6 hrs)
UI has all information that backend has

### (B) Support Multiple Goods (6hrs)
- Remove hardcoded "Grain" good
- Use `Map<Good, MarketInfo>` everywhere a plain `MarketInfo` is currently used

### (C) Visualize Current Time-Step (8 hrs)
- Enable different buttons to change what is displayed in table (1 hr)
- Create table showing all agents (2 hr)
	- money (w/ change), current good, location, previous location, ranking by money (w/ change)
- Create table showing specific good across all cities (1 hr)
- Display average and std-dev of price of goods across cities (w/ change) (2 hr)
	- Make always visible in header bar

### (D) Configure Scenario from Frontend (12 hrs)
- If backend starts up with no scenario defined, wait for frontend (4hrs)
	- Make scenario config struct optional 
	- Add configuration endpoint
	- Block on receiving config 
- Add 'upload config' button (3 hrs)
	- convert yaml to json
	- send to backend endpoint
- Create form that populates config (4 hrs)
	- Look for framework that auto generates the form from a schema

### (E) Hosting (8-?? hrs) 
- Research cheap and easy hosting provider (2 hrs)
	- Heroku
	- Other?
- Dockerize app (2 hrs)
- Deploy (4 hrs)
- Debug (??)
- (stretch) Achieve continuous deployment with github actions 
- (stretch) Buy real domain name and connect it to hosting service

### (F) Visualize Across Time-Steps (8 hrs)
- (done) Store state indexed by time steps 
- Create toggle-able box with chart (3 hrs)
- Plot money over time per agent (1 hrs)
- Plot price over time (2 hrs)
	- all goods for 1 city
	- 1 good for all cities
- Plot efficiency of market over time (2 hrs)
	- All goods combined
	- Individual goods 

### (G) Deepen Simulation (?) 
- Agents can predict multiple steps
	- (Simplification) use current prices 
	- DFS to find actions that maximizes profit
	- States are defined by action stack
		- buy, sell, move
- Support integer edge distances (requires multi-step prediction)
	- When movement takes >1 tick, agent's position changes to `Edge` instead of `Node`
	- No actions are possible when in `Edge`, but a system marks progress to reaching destination

## Timeline
Total time is >48 hrs
Wed 11/17, Thurs 11/18 - 16 hrs
Random time at Pensacola - 8 hrs
Tues 11/30 - 8 hrs
Thurs 12/2, Fri 12/3 - 16 hrs
Full days = 40 hrs
