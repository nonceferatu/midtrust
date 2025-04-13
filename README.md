"about" => html! {
    <>
        <h4 class="mb-3">{"About MidTrust"}</h4>
        <p>
            {"MidTrust is a peer-to-peer escrow simulator designed to help two users — a sender and a receiver — complete a transaction securely without relying on trust or intermediaries. "}
            {"It’s inspired by zero-knowledge cryptography, where users can prove they know something without revealing the actual data."}
        </p>

        <h5 class="mt-4">{"🔄 What Happens Behind the Scenes"}</h5>
        <ol>
            <li>
                {"The sender fills in their wallet, the receiver’s wallet, and a shared secret — think of this secret like a one-time password or deal passcode."}
            </li>
            <li>
                {"The system immediately hashes the secret using a secure algorithm (SHA256). A hash is like a digital fingerprint: it uniquely represents the input but can't be reversed to reveal the secret itself."}
            </li>
            <li>
                {"This hashed secret is stored along with the sender, receiver, and deal amount as a new deal marked 'Pending'."}
            </li>
            <li>
                {"The receiver later submits their version of the secret. The system hashes that input and compares it to the original stored hash."}
            </li>
            <li>
                {"If it matches exactly, the system updates the deal's status from 'Pending' to 'Released', indicating the transaction has been successfully completed."}
            </li>
        </ol>

        <p>
            {"At no point is the actual secret stored or visible to the system — only its hash is kept. This mimics the way zero-knowledge proofs work: proving something is correct, without exposing the sensitive input itself."}
        </p>

        <h5 class="mt-4">{"🔐 Why It’s Trustless"}</h5>
        <p>
            {"The receiver can’t fake the deal because only the correct secret — known in advance — will generate the correct hash. "}
            {"And the sender doesn’t have to manually confirm anything, because the proof (the matching hash) does that verification automatically."}
        </p>

        <h5 class="mt-4">{"🌐 Use Cases"}</h5>
        <ul>
            <li>{"Secure freelance payments that only go through once the work is delivered"}</li>
            <li>{"Deposits for housing or rentals that unlock once terms are met"}</li>
            <li>{"Item or digital asset trades (games, NFTs, files)"}</li>
            <li>{"DAO-based micro agreements, promises, or rewards"}</li>
        </ul>
    </>
},
_ => html! { <p>{"Invalid tab"}</p> }
