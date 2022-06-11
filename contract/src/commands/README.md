# Commands for the NEAR Support App 

## The smart contract is deployed at:

`support.maingi9.testnet`

## Client deposit method

`near call support.maingi9.testnet deposit --account_id maingi9.testnet --deposit 10`

## Client get deposit method

`near view support.maingi9.testnet get_deposit '{"account_id": "maingi9.testnet"}' --account_id maingi9.testnet`

## Client send gift method

`near call support.maingi9.testnet send_gift '{"youtube_user_id": "youtuber.maingi9.testnet", "token": "4000000000000000000000000"}' --account_id maingi9.testnet`

## Content Creator get balance method

`near view support.maingi9.testnet get_balance '{"youtube_user_id": "youtuber.maingi9.testnet"}' --account_id maingi9.testnet`
