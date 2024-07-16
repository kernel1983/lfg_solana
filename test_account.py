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

app_account_bytes = json.loads(open('./app_account.json', 'rt').read())
app_account = solders.keypair.Keypair.from_bytes(app_account_bytes)
print(app_account.pubkey())

# 生成密钥对
# app_account = Keypair()
# new_account = Pubkey.new_unique()
program_id = solders.pubkey.Pubkey.from_string('EAmZj4ctukjgLsp7okQ8R4Yzi4rtuWZEPVCs6nTi2mry')
account_size = 1

instruction = solders.system_program.create_account(
    solders.system_program.CreateAccountParams(
        from_pubkey=sender.pubkey(),
        to_pubkey=app_account.pubkey(),
        lamports=1000000,
        space=account_size,
        owner=program_id
    )
)

tx = solana.transaction.Transaction()
tx.add(instruction)
ret = http_client.send_transaction(tx, sender, app_account)
# print(ret)
# print(keypair.pubkey())