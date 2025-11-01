import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MessageSaver } from "../target/types/message_saver";

describe("message_saver", () => {
  // Configure client to use the local cluster
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.MessageSaver as Program<MessageSaver>;

  it("Saves a message on-chain", async () => {
    const [messageAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("message"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .saveMessage("Hello Solana Blockchain!")
      .accounts({
        messageAccount,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Transaction signature:", tx);

    const account = await program.account.messageAccount.fetch(messageAccount);

    console.log("Stored message:", account.text);
    if (account.text !== "Hello Solana Blockchain!") {
      throw new Error("Message mismatch!");
    }
  });
});
