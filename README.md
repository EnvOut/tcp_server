### Test task for Server Engineer

Design and implement “Word of Wisdom” tcp server.
• TCP server should be protected from DDOS attacks with the Prof of Work (https://en.wikipedia.org/wiki/Proof_of_work), the challenge-response protocol should be used.
• The choice of the POW algorithm should be explained.
• After Prof Of Work verification, server should send one of the quotes from “word of wisdom” book or any other collection of the quotes.
• Docker file should be provided both for the server and for the client that solves the POW challenge

Proof of work - Wikipedia
en.wikipedia.org • Для прочтения потребуется 7 мин.
вот наше тестовое задание, его нужно выполнить на rust. Когда сможете его сделать? у нас нет сроков, просто спрашиваю временные рамки

### Implemented:
- [X] Tcp connection on the client and server side
- [X] Doss protection based on [Hashcash](https://en.wikipedia.org/wiki/Hashcash)
- [X] Communication flow in two requests:
   - AskPazzle - for fetching new pazzle from server
   - GetResource - for fetching resource after finding answer
- [X] Routing implemented, client can select resource and get related data 
- [X] Implemented stateless answer verification using encripted data inside "ext" part
- [X] Prepared docker files and docker-compose file


### Required improvements:
- [ ] use more efficient binary format for request/response data
- [ ] improve server / client communication implimentation
- [ ] use config / env file
- [ ] improve tests coverage

### Run
1. Install rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. run server: `cargo run --bin server`
3. run client: `cargo run --bin client`

### Running with docker / docker-compose
1. Install Docker and Docker Compose
2. docker-compose up