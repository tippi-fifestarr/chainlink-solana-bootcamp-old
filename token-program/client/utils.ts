/* utils.ts. These helper functions will be used to get config stored locally, 
get the RPC URL required to connect to a cluster, as well as generate a new keypair to be used for interacting with the deployed program.

Note: If you aren’t running a local validator and are deploying to the public Devnet, 
please replace the ‘http://127.0.0.1:8899’ string in the getRpcUrl() function with ‘https://api.devnet.solana.com’ */

import os from 'os';
import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import {Keypair} from '@solana/web3.js';

/**
* @private
*/
async function getConfig(): Promise<any> {
 // Path to Solana CLI config file
 const CONFIG_FILE_PATH = path.resolve(
   os.homedir(),
   '.config',
   'solana',
   'cli',
   'config.yml',
 );
 const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
 return yaml.parse(configYml);
}

/**
* Load and parse the Solana CLI config file to determine which RPC url to use
*/
export async function getRpcUrl(): Promise<string> {
   return 'http://127.0.0.1:8899';
}

/**
* Load and parse the Solana CLI config file to determine which payer to use
*/
export async function getPayer(): Promise<Keypair> {
 try {
   const config = await getConfig();
   if (!config.keypair_path) throw new Error('Missing keypair path');
   return await createKeypairFromFile(config.keypair_path);
 } catch (err) {
   console.warn(
     'Failed to create keypair from CLI config file, falling back to new random keypair',
   );
   return Keypair.generate();
 }
}

/**
* Create a Keypair from a secret key stored in file as bytes' array
*/
export async function createKeypairFromFile(
 filePath: string,
): Promise<Keypair> {
 const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
 const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
 return Keypair.fromSecretKey(secretKey);
}