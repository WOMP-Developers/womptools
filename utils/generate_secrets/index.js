const util = require('util');
const exec = util.promisify(require('child_process').exec);
const { v4: uuidv4 } = require('uuid');
const { pem2jwk } = require('pem-jwk');
const selfsigned = require('selfsigned');
const crypto = require('crypto');
const fs = require('fs');

const KEYS_DIRECTORY = "../../keys";

function generate_refresh_token_secret() {
    return crypto.randomBytes(64).toString('base64');
}

function generate_access_token_keys() {
    const attr = [{name: 'commonName', value: 'womptools'}];
    const opts = { keySize: 2048 };

    const { private: privatekey, public: publickey } = selfsigned.generate(attr, opts);

    return { 
        privatekey,
        publickey
    };
}

function generate_access_token_jwk(publickey) {
    const kid = uuidv4();

    const extras = {
        "kid": kid,
        "alg": "RS512",
        "use": "sig"
    };

    return pem2jwk(publickey, extras);
}

async function generate_keys() {
    fs.mkdirSync(KEYS_DIRECTORY, { recursive: true });

    const { privatekey, publickey } = generate_access_token_keys();
    console.log("generated access token keys")

    const jwk = generate_access_token_jwk(publickey);
    console.log("generated access token jwk")

    const jwks = { keys: [ jwk ] };

    fs.writeFileSync(`${KEYS_DIRECTORY}/access_token.private.pem`, privatekey);
    fs.writeFileSync(`${KEYS_DIRECTORY}/access_token.public.pem`, publickey);
    fs.writeFileSync(`${KEYS_DIRECTORY}/jwks.json`, JSON.stringify(jwks));

    // const refresh_token_secret = generate_refresh_token_secret();
    // console.log("generated refresh token secret");

    // fs.writeFileSync(`${KEYS_DIRECTORY}/refresh_token_secret`, refresh_token_secret);
}

generate_keys();
