# Sending Messages Anonymously via Trusted Relay Intermediate

_Look how you want, the only thing you can see is code_

This is an attempt to build a simple yet fully anonymous platform for sending and receiving messages.

The anonymity is achieved by using a trusted relay (currently oblivious.network, but Tor/I2P usage is certainly possible with the same effect),
which guarantees to erase the origin metadata and pass through the request body (currently via Oblivious HTTP protocol described in RFC 9458).

## Architecture

The architecture consists of four component: sender, receiver, trusted relay and the backend, where backend has global state, receiver has only user state and sender is stateless.

Sender and receiver are simple web pages, served via GitHub pages (TODO: GitHub build-attested), which send the encrypted data to relay.

Trusted third-party relay then strips the request metadata away, and forwards the encrypted body to the backend.

Backend decrypts the body, which can be a request of two types:

1. Sender -> Push message (receiver_public_key, encrypted_content)
2. Receiver -> Pull messages (receiver_public_key)

Backend stores the messages in a simple SQLite database and queries them as requested.

Upon loading, receiver page generates a private-public key pair and stores it securely. The public key is then encoded into a sender link in such a way that does NOT leak into the HTTP request.
After that, it periodically (with a randomized interval) fetches the messages by its public key pair via relay, decrypts them via its private key and shows the result.

Upon its loading, sender extracts the receiver_public_key from the URL, prompts the message, encrypts it with the public key, then encrypts it with the backend key and sends it to the relay. Messages are padded to a fixed size of 4K.

## Implementation

* Sender, Receiver are HTML + JS + Rust-WASM pages with bundled relay address and backend public keys, using ohttp to connect to the underlying relay.
* Receiver stores the keypair in IndexedDB, and generates a link using a location-tag (https://example.com/send#PUBLIC_KEY_HEX).
* Relay currently uses oblivious.network provider, but any such system suffices
* Backend uses Rust with ohttp to receive and respond to the relay requests, and stores the public_key -> encrypted_content mapping in a SQLite database.

## Security analysis

The main attack this application tries to protect from is sender-deanonymization, where someone gets any meaningful information about who sent the messages except Receiver knowing the messages themselves.

In this design, for an attack extracting some meaningful data after the sender interacts with a correctly-formed sender link to be successful, it has to involve either the sender or the relay, because
the content of the message is encrypted by the receiver key, and the metadata of the message persists only between sender and relay on a secure connection.

Thus, if we can trust the relay to drop all the sensitive metadata, this system is secure to good senders.

Obvious DoS attacks and the possibility of dropping messages by a malicious backend are out-of-scope.

One should also note that the hosting of such page should also be done in a trusted space, or that the page should be verified against a pre-existing signature before running it.

