<p align="center">
  <img src="https://github.com/austinmehmet/terrafarm/blob/master/assets/terrafarm.png" alt="terrafarm-logo" width="120px" height="120px"/>
  <br>
  <i>Terrafarm is a fork-able Rust based CLI that allows companies to provide their developers
    <br> with easy-to-access terraform files</i>
  <br>
</p>


## Why? 
I've always hated starting Terraform files from scratch. Instead of painfully starting a blank `main.tf`, I would almost always go hunting for starter files off of the terraform website, StackOverflow, or Github only to find that these starter templates do not follow best practices my company has chosen to follow with things like tagging, naming conventions, security configurations etc. Instead of forcing developers to hunt for these standards through your internal repos, why not provide them with templates that can act as a starting point for their applications? A series of terraform starter files for common services can provide the much needed lift a team requires to get into the cloud ASAP. Terrafarm is all about organizing all your sharable Terraform code into a central repo and 'farm'ing them through simple commands.
 
## Example CLI Usage
```shell
farm get -p aws -s dynamodb
 
farm get -p aws -s api_gateway
 
farm get -p gcp -s cloudfunctions
 
farm get -p azure -s iothub

farm get -p patterns -s spa
```
 
## Base Functionality
 
Out of the box this CLI will work and query template files for a ton of cloud provider services but that is not really the goal of this CLI as for the majority of these services your company will require additional configuration or make use of customized modules. So while you can use the CLI now to query generic templates, you should instead push your company to fork this repo and providing their own internally deployed solution.
 
## What do I need to do to roll my own Terrafarm?
1. Create an s3 bucket or other hosting solution and add any of your company's terraform templates into it following the convention of `/bucket/provider/service` but if you didn't want to follow that I can't stop you
2. Modify the included `src/dictionary.json` file to point to your own company's internal templates, or don't
3. Bundle up this program and distribute it through your company. To avoid conflicts with this CLIs deployment, it would be recommended to scope your CLI to `@company/farm` but again, do whatever, I am helpless to whatever choices you make
 
## What about combining services for more real life examples like a SPA deployment or a Lambda API?
The power of this CLI is that it can be configured to your liking seeing you do most of the work anyway by building the dictionary. If you check the `src/dictionary.json` there is a section named `patterns` where you can list out your own customize commands. Dictionary items can refer to either a single URI or an array of URI allowing complex applications like single page application deployments to be handled with ease.
 
## Do I really need this CLI?
 
Not at all, someone could just copy and paste the files if you were to publish your `dictionary.json` somewhere. This is just convenient and nothing more. Now, this CLI could do a lot more. Provided a series of extra arguments like `farm configure -p aws -s dynamodb --name="my-table" --read-units="30" --write-units="15" --hash-key="id" --range-key="title"` you could build a CLI where not only do your developers get a template but they get a template with (hopefully) no extra needed customization.

## YIPPEE I never need to write Terraform code again!!!

Hold up there buddy, this CLI should only give developers a starting point. It would not be a fair assumption that any of the tempaltes you download from this CLI will `tf apply` right out of the gate. 
