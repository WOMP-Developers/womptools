# WOMP Tools

A web application for tracking multiple Eve Online toons.

## 💡 Features

### Authentication
Authenticate Eve Online toons and tie them all to a "main" account. The web application never expose the Eve ESI tokens externally and instead uses JWT access/refresh tokens for authorization. 

### Wallet Tracking
Track total wallet balance across all characters. Also presents various statistics related to wallets.
* Ratting bounties
* Income / Expenses graph
* Total balance graph
* Wallet journal list

### More features to be added
* Assets
* Blueprints/Industry
* PI
* Skill Queue


## 🛠️ Development

### Dependencies
* Docker / Kubernetes Cluster
* Nodejs - Secret generator uses nodejs
* Rust - Microservices

### Running for development

#### Generating Secrets
After ensuring all the dependencies is installed and ready to be used secrets must be generated. These secrets are used to sign the JWT's used for authentication and authorization. There's a utility project which will generate the keys in the expected format.

Navigate to the `utils/generate_secrets` directory and run the Nodejs javascript there. This script will generate files in the `keys` directory.

```sh
cd utils/generate_secrets

npm install
npm run start
```

Double check the following files appeared:

```
keys/access_token.private.pem
keys/access_token.public.pem
keys/jwks.json
```

Do no share these secret keys with anyone.

#### Setup Kubernetes Cluster
The databases, message queue, and monitoring services run within kubernetes. Create the necessary resources within kubernetes by running the following command:
```sh
kubectl apply -f k8s/
```

During development most services will be exposed externally using `NodePort`. Examine the assigned ports for each service using the following command:
```sh
kubectl get service -n womptools
```

These exposed ports is necessary later when configuring each microservice environment. The ports will be persistent until the Kubernetes cluster or resources is recreated so this is mostly a one time step/configuration.

#### Running Microservices
The microservices is located inside the `./services/` directory. Go through each one of the directories and copy the `.env.example` file to `.env`. Then update the variables to the correct values based on kubernetes `NodePorts` and service port assignments.

For example:
```sh
cd services/users
cp .env.example .env

# Edit the .env file
vim .env
```

Running the services is done using the Rust `cargo` utility. Navigate to the service directory and use `cargo run`.

```sh
cd services/users
cargo run

cd services/sso
cargo run

cd services/characters
cargo run

cd services/wallet
cargo run
```


#### Running Frontend
Similarly to the microservices the frontend is configured using environment file. Navigate to the `./womp-tools` directory and copy the `.env.example` file to `.env.local`. Update the variables with the correct values.

```sh
cd womp-tools
cp .env.example .env.local

# Edit the .env.local file
vim .env.local
```

The frontend project is running using `nodejs` so the dependencies has to be installed and then the project can be started.
```sh
# Install dependencies
npm install

# Run project
npm run dev

```

If everything was configured correctly it should now be possible to open the tool at `http://localhost:3000` and login using Eve ESI SSO.
