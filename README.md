Purpose of this Backend is to fix scalability issues in our Program Driven Account (PDA) defined
in https://github.com/chancecardona/sol-stream-backend
(namely we wish to serve the PDAs at given routes with Rocket.rs,
index them so we can find the 2 relevant streams, as PDA's = (# of streams) * (# of users)) 
via SQL with Deisel.rs,

