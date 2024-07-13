import json

from solders.keypair import Keypair
from solders.pubkey import Pubkey
import solders.system_program
from solana.transaction import Transaction

import solana.rpc.api
import solana.transaction
import spl.token.client
import spl.token.instructions
import spl.token.constants

http_client = solana.rpc.api.Client("http://127.0.0.1:8899")

sender_bytes = json.loads(open('/home/ubuntu/.config/solana/id.json', 'rt').read())
sender = solders.keypair.Keypair.from_bytes(sender_bytes)
print(sender.pubkey())

# 生成密钥对
keypair = Keypair()
program_id = solders.pubkey.Pubkey.from_string('EAmZj4ctukjgLsp7okQ8R4Yzi4rtuWZEPVCs6nTi2mry')

# 确定账户大小
# account_size = 1024

instruction = solders.system_program.create_account(
    solders.system_program.CreateAccountParams(
        from_pubkey=sender.pubkey(),
        to_pubkey=keypair.pubkey(),
        lamports=10000000,
        space=0,
        owner=program_id
    )
)

tx = solana.transaction.Transaction()
tx.add(instruction)
ret = http_client.send_transaction(tx, sender, keypair)
# print(ret)
print(keypair.pubkey())