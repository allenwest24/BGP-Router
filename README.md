# BGP-Router-Implementation

## Project Description:
In reality, a BGP router is a hardware box that has a bunch of fancy, high-speed jacks (or ports) on it. First, a network administrator would plug cables into these ports that connect to neighboring BGP routers, either from the same Autonomous System (AS) or another AS. Second, the administrator would manually configure each of these ports by: 
- Choosing the IP address that the router will use on this port, since each port will have a different IP 
- Choosing whether this port leads to a provider, a peer, or a customer (i.e. what kind of BGP relationship does the router have with each neighbor?) 
- Possibly manually configuring some specific routes via each neighbor 
Once this manual configuration is complete, the administrator would turn the router on, at which point it will contact its neighbors and establish BGP sessions. At this point, the neighboring routers can pass BGP protocol messages to each other, as well as pass data packets from Internet users. The routers job is to: 
- Keep its forwarding table up-to-date, based on the BGP protocol messages it gets from its neighbors 
- Help keep its neighbors' forwarding tables up to date, by sending BGP protocol messages to them 
- Make a best-effort attempt to forward data packets to their correct destination
This program acts like a router. When router is executed, it opens several Unix domain sockets, each of which corresponds to one "port" in the router.The router is able to receive messages on these sockets which will either be BGP commands from a given neighbor, or a data packet that the router will then forward to the correct destination.

## This Router's Capabilities:
- Accepts route update messages from BGP neighbors, and forwards updates to neighbors based on stored relationships for those neighbors and the peer who sent the update (Relationships include peer, customer, and provider).
- Accepts route revocation messages from BGP neighbors, and forwards simliarly to update messages.
- Forwards data packets towards their correct destination.
- Returns error messages in cases where a data packet cannot be delivered.
- Coalesces forwarding table entried for networks that are adhacent and on the same port.
- Maintains a serialized routing table, relationships table, update announcement table, and open sockets tables.

## Demo This Router:
- Download this repository on your own system.
- cd into this directory on your own system.
- Call "./sim all" to run all the tests provided.
- Inspect the tests in the ./tests folder and see what kind of json inputs there are.

## Challenges:
- A major challenge I faced was in my previous implementation of self.routes and self.updates as arrays. I had to go back and re-instate them as structs. This was a mistake on my part an took a minute to correct.
- Also, when implementing the 5 rules I found significant troubles in how I paired key values to the different routes and update packages. Once I settled on a peer->network structure for the key, it was smooth sailing. Previously, routes were getting overwritten but now they would no longer be.
- Figuring out the ip->binary was a little tricky but I took the approach of separating by the '.'s and comparing individual sections as binary.
- A major confusion was about what was classified as neighboring IPs and once I figured out that they had to have the same netmask, the rest was fairly easy.
- Another hard part in the aggregate method was the fact that once two parts were aggregated, they could also possibly be aggregated further, so we had to add an additional check to see if it worked and if there was any follow-on action needed.

## Testing:
Testing for this project was largely handled through the tests in the ./tests folder. If things were not working, I would consult the error messages and try again. A massive portion of testing was through print statements everywhere. Checking if the routing table consistently looked how I wanted it to look was a major challenge so this is how I tested this. Other than that, I mainly just used the tests and made sure previous tests were not broken by new features at the end of each level of tests.
