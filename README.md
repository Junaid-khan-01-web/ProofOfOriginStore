## Project Title
TodoAPI Service

## Project Description
TodoAPI Service provides a simple onchain task registry that supports creating tasks, marking them complete and viewing tasks. Each task stores an owner, title, details, done flag and a creation timestamp. Owners can manage their tasks through simple contract calls.

## Project Vision
The vision is to offer a verifiable, auditable source of truth for lightweight task management and workflows where immutability or tamperproof history is valuable. Offchain APIs can wrap the contract to present a RESTful interface while the contract maintains authoritative state.

## Key Features
Create tasks with a title and detail and store them onchain. Mark a task as done while enforcing owner permissions. Retrieve a task by identifier including owner and timestamp. The contract is compact so it can be integrated into a backend that exposes REST endpoints.

## Future Scope
Add task editing, deadlines, priority and assignments. Integrate token incentives for task completion, notifications via oracles and paginated owner task queries for efficient UI consumption.

## Contract Details
Contract ID: CD7YCFPMRO2ZPMOGLUEUZQN2ZCVEMNR62CTRG5GS6TRDG3RCTCKHQZXD
![alt text](image.png)