import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OfflineSigning } from "../target/types/offline_signing";

describe("offline-signing", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OfflineSigning as Program<OfflineSigning>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
