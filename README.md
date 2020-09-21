## dumplack

dumplack is mysql dump slack notify

## Requirement

 - mysqldump
 - masking

## Usage

 - command run local

```
SLACK_HOOK="hook_url" SLACK_CHANNEL_NAME="channel_name" BUCKET_NAME="xxx" HOST="hostname" PORT=12345 USER_NAME="username" PASSWORD="password" SCHEMA="schema" aws-vault exec profile -- cargo run
```