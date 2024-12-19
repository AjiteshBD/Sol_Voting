import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {Votingdapp} from '../target/types/Votingdapp'
import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

const IDL = require('../target/idl/Votingdapp.json')

const votingAddress = new PublicKey("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF")

describe('Votingdapp', () => {
  // Configure the client to use the local cluster.
  

  it('Initialize Poll', async () => {
     const context = await startAnchor("",[{name:"Votingdapp", programId:votingAddress}],[])
     const provider = new BankrunProvider(context);

     const votingProgram = new Program<Votingdapp>(
      IDL,
      provider
    );

    

    await votingProgram.methods.initializePoll(
      new anchor.BN(1),
      "Trump vs Modi?",
      new anchor.BN(0),
      new anchor.BN(1834589885)
    ).rpc(); 

    const [pollAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("Poll"),new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      votingProgram.programId,
    )

    const poll = await votingProgram.account.pollAccount.fetch(pollAddress)
    console.log(poll)

    expect(poll.pollId.toNumber()).toEqual(1)
    expect(poll.description).toEqual("Trump vs Modi?")
    expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber())
 

  })

  
})


//   it('Initialize Poll', async () => {
//     const context = await startAnchor("", [{ name: "Votingdapp", programId: votingAddress }], []);
//     const provider = new BankrunProvider(context);

//     const votingProgram = new Program<Votingdapp>(
//       IDL,
//       provider
//     );

 


//     await votingProgram.methods
//       .initializePoll(
//         new anchor.BN(1),
//         "Trump vs Modi?",
//         new anchor.BN(0),
//         new anchor.BN(1834589885)
//       )
//       .rpc();

//       const [pollAddress, pollBump] = PublicKey.findProgramAddressSync(
//         [Buffer.from("Poll"),new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
//         votingProgram.programId,
//       );

//     const poll = await votingProgram.account.pollAccount.fetch(pollAddress);
//     console.log("Fetched Poll Account:", poll);
//   });
// });
