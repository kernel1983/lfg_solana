import json
import time

import solders.keypair
import solders.pubkey
import solders.system_program

import solana.rpc.api
import solana.transaction
import spl.token.client
import spl.token.instructions
import spl.token.constants

def main():
    http_client = solana.rpc.api.Client("http://127.0.0.1:8899")

    program_id = solders.pubkey.Pubkey.from_string('EAmZj4ctukjgLsp7okQ8R4Yzi4rtuWZEPVCs6nTi2mry')

    sender_bytes = json.loads(open('/home/ubuntu/.config/solana/id.json', 'rt').read())
    sender = solders.keypair.Keypair.from_bytes(sender_bytes)
    print(sender.pubkey())

    # create app account
    bins = 5
    account_size = 40 * bins
    seed = b'app'
    # app_pubkey = solders.pubkey.Pubkey.create_with_seed(sender.pubkey(), seed, program_id)
    app_pubkey, bump = solders.pubkey.Pubkey.find_program_address(seeds=[seed], program_id=program_id)
    print('app account', app_pubkey, 'bump', bump)
    # address = solders.pubkey.Pubkey.create_program_address([seed, bytes([bump])], program_id)
    # print('address', address)

    # account_info = http_client.get_account_info(app_pubkey)
    # print(account_info)
    # if not account_info.value:
    #     instruction = solders.system_program.create_account_with_seed(
    #         solders.system_program.CreateAccountWithSeedParams(
    #             from_pubkey=sender.pubkey(),
    #             to_pubkey=app_pubkey,
    #             base=sender.pubkey(),
    #             seed=seed,
    #             lamports=10000000,
    #             space=account_size,
    #             owner=program_id
    #         )
    #     )

    #     tx = solana.transaction.Transaction()
    #     tx.add(instruction)
    #     ret = http_client.send_transaction(tx, sender)
    #     print(ret)

    #     while not account_info.value:
    #         print('waiting app account')
    #         time.sleep(3)
    #         account_info = http_client.get_account_info(app_pubkey)

    # price in sol         u64   8 bytes
    # token total         u128  16 bytes
    # token amount        u128  16 bytes
    #                           40 bytes
    bins_data = b''
    for i in range(bins):
        print(i)
        bins_data += (10**18).to_bytes(8, byteorder='little') # 10 lamport per token
        bins_data += (10**18 * 2).to_bytes(16, byteorder='little')
        bins_data += (0).to_bytes(16, byteorder='little')

    print('setup instruction')
    setup_instruction_data = bytes([0])
    setup_instruction_data += len(seed).to_bytes(1, byteorder='little')
    setup_instruction_data += seed
    setup_instruction_data += (bump).to_bytes(1, byteorder='little')
    setup_instruction_data += bins_data
    print('setup instruction data', setup_instruction_data)

    accounts = [
        solders.instruction.AccountMeta(sender.pubkey(), True, True),
        solders.instruction.AccountMeta(app_pubkey, False, True),
        # solders.instruction.AccountMeta(user_pubkey, False, True),
        solders.instruction.AccountMeta(program_id, False, False),
        solders.instruction.AccountMeta(solders.system_program.ID, False, False),
    ]
    setup_instruction = solders.instruction.Instruction(program_id, setup_instruction_data, accounts)
    tx = solana.transaction.Transaction()
    tx.add(setup_instruction)
    ret = http_client.send_transaction(tx, sender)
    print(ret)

    account_info = http_client.get_account_info(app_pubkey)
    print(account_info)
    while not account_info.value:
        print('waiting for app account creating')
        time.sleep(3)
        account_info = http_client.get_account_info(app_pubkey)

if __name__ == '__main__':
    main()
