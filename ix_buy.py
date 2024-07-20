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
    seed = 'app'
    app_pubkey = solders.pubkey.Pubkey.create_with_seed(sender.pubkey(), seed, program_id)
    print('app account', app_pubkey)
    account_info = http_client.get_account_info(app_pubkey)
    print(account_info)
    if not account_info.value:
        instruction = solders.system_program.create_account_with_seed(
            solders.system_program.CreateAccountWithSeedParams(
                from_pubkey=sender.pubkey(),
                to_pubkey=app_pubkey,
                base=sender.pubkey(),
                seed=seed,
                lamports=10000000,
                space=account_size,
                owner=program_id
            )
        )

        tx = solana.transaction.Transaction()
        tx.add(instruction)
        ret = http_client.send_transaction(tx, sender)
        print(ret)

        while not account_info.value:
            print('waiting app account')
            time.sleep(3)
            account_info = http_client.get_account_info(app_pubkey)


    # create user account
    account_size = 8
    seed = 'user'
    user_pubkey = solders.pubkey.Pubkey.create_with_seed(sender.pubkey(), seed, program_id)
    print('user account', user_pubkey)
    account_info = http_client.get_account_info(user_pubkey)
    print(account_info)
    if not account_info.value:
        instruction = solders.system_program.create_account_with_seed(
            solders.system_program.CreateAccountWithSeedParams(
                from_pubkey=sender.pubkey(),
                to_pubkey=user_pubkey,
                base=sender.pubkey(),
                seed=seed,
                lamports=10000000,
                space=account_size,
                owner=program_id
            )
        )

        tx = solana.transaction.Transaction()
        tx.add(instruction)
        # ret = http_client.send_transaction(tx, sender, app_account)
        ret = http_client.send_transaction(tx, sender)
        print(ret)
        # print(keypair.pubkey())
        # return
        # wait until account created

    while not account_info.value:
        print('waiting user account')
        time.sleep(3)
        account_info = http_client.get_account_info(user_pubkey)

    print('instruction')
    arbitrary_instruction_data = bytes([1])
    lamports = 10**9 # 1 sol
    arbitrary_instruction_data += lamports.to_bytes(8, byteorder='little')
    print('instruction data', arbitrary_instruction_data)

    accounts = [
        solders.instruction.AccountMeta(sender.pubkey(), True, False),
        solders.instruction.AccountMeta(app_pubkey, False, True),
        solders.instruction.AccountMeta(user_pubkey, False, True),
        solders.instruction.AccountMeta(solders.system_program.ID, False, False),
    ]
    instruction = solders.instruction.Instruction(program_id, arbitrary_instruction_data, accounts)
    tx = solana.transaction.Transaction()
    tx.add(instruction)
    ret = http_client.send_transaction(tx, sender)
    print(ret)

    # arbitrary_instruction_data = bytes([2, 0, 0, 0, 0, 0, 0, 0, 0])
    # accounts = [
    #     solders.instruction.AccountMeta(solders.pubkey.Pubkey.from_string('GmkqdfZd1MzatuPbSNbe2RshuakKKmGreDbSeUEZaJ3z'), False, True),
    #     solders.instruction.AccountMeta(sender.pubkey(), True, False),
    #     # solders.instruction.AccountMeta(solders.system_program.ID, False, False),
    # ]
    # instruction = solders.instruction.Instruction(program_id, arbitrary_instruction_data, accounts)
    # tx = solana.transaction.Transaction()
    # tx.add(instruction)
    # http_client.send_transaction(tx, sender)


if __name__ == '__main__':
    main()
