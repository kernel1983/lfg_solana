import json

import solders.keypair
import solders.pubkey
import solders.system_program

import solana.rpc.api
import solana.transaction
import spl.token.client
import spl.token.instructions
import spl.token.constants

http_client = solana.rpc.api.Client("http://127.0.0.1:8899")

# print(http_client.get_latest_blockhash().context.slot)
# print(http_client.get_latest_blockhash().value.blockhash)

sender_bytes = json.loads(open('/home/ubuntu/.config/solana/id.json', 'rt').read())
sender = solders.keypair.Keypair.from_bytes(sender_bytes)
print(sender.pubkey())

program_id = solders.pubkey.Pubkey.from_string('EAmZj4ctukjgLsp7okQ8R4Yzi4rtuWZEPVCs6nTi2mry')
arbitrary_instruction_data = bytes([0, 0, 0, 0, 0, 0, 0, 0, 0])
accounts = [
    solders.instruction.AccountMeta(sender.pubkey(), True, True),
    solders.instruction.AccountMeta(solders.pubkey.Pubkey.from_string('CBKaFSNvoth4uFDZUnpUGjZE5QrzRebMG9GFn1wjw18C'), False, True),
    solders.instruction.AccountMeta(solders.system_program.ID, False, False),
]
instruction = solders.instruction.Instruction(program_id, arbitrary_instruction_data, accounts)
tx = solana.transaction.Transaction()
tx.add(instruction)
http_client.send_transaction(tx, sender)


# token_client = spl.token.client.Token.create_mint(
#     http_client,
#     sk,
#     sk.pubkey(),
#     6,
#     spl.token.constants.TOKEN_PROGRAM_ID,
# )
# print(token_client)
# print(token_client.pubkey)
# print(dir(token_client))

# sk2_list = json.loads(open('2.json', 'rt').read())
# sk2 = solders.keypair.Keypair.from_bytes(sk2_list)
# print(sk2.pubkey())
# receiver = sk2

# transaction = solana.transaction.Transaction().add(spl.token.instructions.transfer(spl.token.instructions.TransferParams(
#         program_id=spl.token.constants.TOKEN_PROGRAM_ID,
#         source=solders.pubkey.Pubkey.from_string('7ouEv3z8XkXo9Z7aHYFLcHakYjKrN2daENq8RFsi8Ffu'),
#         dest=solders.pubkey.Pubkey.from_string('GmkqdfZd1MzatuPbSNbe2RshuakKKmGreDbSeUEZaJ3z'),
#         amount=10**9,
#         owner=sender.pubkey(),
#     )
# ))

# http_client.send_transaction(transaction, sender)


# spl-token accounts
# spl-token create-token 
# 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA
# spl-token create-account 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA
# spl-token mint 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA 10