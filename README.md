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

## High-Level Approach:

### Milestone:
- Before writing any code we took a look at some test cases. This was primarily to familiarize ourselves with what kind of inputs we would be dealing with.
- We essentially created a make-shift switch to handle each type of packet that could be recieved, and passed on the packet with the srcif it came from.
- First packet-specfic implementation was for update. We based the routing table on what would information would be necessary upon a "dump" request. We fullfilled the 3 requirements for an update: saving a copy to self.updates, updating the routing table in self.routes, and sending the update message along to neighboring routers according to who sent the packet. We did this by cross-referencing the self.relations table.
- From here, we handled the second packet-specific type, data packets. These ones need to be sent through our router to our route (right now we assumed one option for each data packet.). 
- The third packet-specific case we had to handle was the "dump" packets, where we had to dump the state of our routing table. For this one, we just read each key value in the self.routes table into an array and put it into the MESG portion of a dump packet and sent it back where it came from.
- Milestone 1 completed: We could now start up a Router incident, open multiple sockets, and handle Update, Data, and Dump packets.

### Post-Milestone:
- The first step we took after completing the level-1 tests was implement the rules needed to pass the level-2 tests. At this point, we found it easier to re-implement self.routes and self.updates as dicts instead of arrays like we had been doing.
- We tested the various rules and saw to it that they were working to filter out routes if there were multiple possibilities.
- Immediatly after this we skipped to the "enforcing peer and provider/customer relationships" because we saw this as a sort of filter as well.
- With the level-2 tests all passing we moved on to the revoke messages.
- We implemented the packet handling for revoke messages fairly easy and were able to send back a revoke message very easily. 
- We also implemented send_error to handle if there was no available routes to the requested destination for a data packet. 
- Here we started in on longest prefix match, or as our method was named; lpfm().This was a failry difficult step because we had to convert to binary and compare, but after we found the right way to do that with python string formats, it was easy.
- Now we started in on the aggregation function. This method was long and messy but ended up working well. We will talk about the struggles we faced here in the challenges portion of this document. Essentially we used coalesce to see if all the rules to aggregate two router entries was valid and then used a method aggregate() to combine them. 
- Here we made the distinction of routes and updates within our router and although we were cross-referencing them a lot, we knew that in our de-aggregate method this would be important.
- After getting our aggregate and coalesce methods working we attempted to decoalesce, and although we were able to separate and repopulate the table, we were unable to re-aggregate valid entries.

## Challenges:
- A major challenge we faced was in our previous implementation of self.routes and self.updates as arrays. We had to go back and re-instate them as structs. This was a mistake on our parts an took a minute to correct.
- Also, when implementing the 5 rules we found significant troubles in how we paired key values to the different routes and update packages. Once we settled on a peer->network structure for the key, it was smooth sailing. Previously, routes were getting overwritten but now they would no longer be.
- Figuring out the ip->binary was a little tricky but we took the approach of separating by the '.'s and comparing individual sections as binary.
- A major confusion was about what was classified as neighboring IPs and once we figured out that they had to have the same netmask, the rest was fairly easy.
- Another hard part in the aggregate method was the fact that once two parts were aggregated, they could also possibly be aggregated further, so we had to add an additional check to see if it worked and if there was any follow-on action needed.

## Testing:
Testing for this project was largely handles through the tests in the ./tests folder. If things were not working, we would consult the error messages and try again. A massive portion of testing was through print statements everywhere. Checking if the routing table consistently looked how we wanted it to look was a major challenge so this is how we tested this. Other than that, we mainly just used the provided tests and made sure previous tests were not broken by new features at the end of each level of tests.
