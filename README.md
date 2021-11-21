# Terrafarm
 
I always hated starting Terraform files from scratch. Almost always I'd go hunting for starter files off the terraform website or via stack overflow only to find that these starter templates do not follow best practices my company has chosen to follow with say tagging, naming conventions, etc. Instead of forcing developers to hunt for these standards, provide them with a starting point for their application. A series of terraform templates files can provide the much needed lift a team requires to get into the cloud ASAP. Let's organize all our Terraform templates into a central repo and 'farm' them through simple commands.
 
## Example Usage
```shell
farm dynamo
 
farm api_gateway
 
farm eks
```
 
## What do I need to do?
1. Create an s3 bucket or other hosting solution and add any of your company's terraform templates into it following the convention of `/bucket/provider/service`
2. Create the dictionary.json file this CLI requires
3. Bundle up this program and distribute it through your company
 
### Do I really need this CLI?
 
Not at all, someone could just copy and paste the files if you were to publish your dictionary.json somewhere. This is just convenient and nothing more. Now, this could do a lot more. Provided a series of extra arguments like `farm dynamo --name="my-table" --read-units="30" --write-units="15"` you could build a CLI where not only do your developers get a template but they get a template with (hopefully) no extra needed customization 