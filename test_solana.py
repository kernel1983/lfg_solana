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
    solders.instruction.AccountMeta(sender.pubkey(), True, False),
    solders.instruction.AccountMeta(solders.pubkey.Pubkey.from_string('F6iT4BjgomDVaMd4JG65grQNU3TBiPXkpTRjHZKxTpnJ'), False, True),
    solders.instruction.AccountMeta(solders.system_program.ID, False, False),
]
instruction = solders.instruction.Instruction(program_id, arbitrary_instruction_data, accounts)
tx = solana.transaction.Transaction()
tx.add(instruction)
http_client.send_transaction(tx, sender)

arbitrary_instruction_data = bytes([1, 0, 0, 0, 0, 0, 0, 0, 0])
accounts = [
    solders.instruction.AccountMeta(solders.pubkey.Pubkey.from_string('F6iT4BjgomDVaMd4JG65grQNU3TBiPXkpTRjHZKxTpnJ'), False, True),
    solders.instruction.AccountMeta(sender.pubkey(), True, False),
    # solders.instruction.AccountMeta(solders.system_program.ID, False, False),
]
instruction = solders.instruction.Instruction(program_id, arbitrary_instruction_data, accounts)
tx = solana.transaction.Transaction()
tx.add(instruction)
http_client.send_transaction(tx, sender)





# spl-token accounts
# spl-token create-token 
# 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA
# spl-token create-account 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA
# spl-token mint 5hrKEJcRRRupB3gkQfCs76A2HmonZ6MVwUcr8zoWXvuA 10