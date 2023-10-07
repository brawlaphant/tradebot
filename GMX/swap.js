const Web3 = require('web3');
const { ethers } = require('ethers');
const axios = require('axios'); // Import axios for HTTP requests

// Initialize a web3 instance
const provider = new ethers.providers.JsonRpcProvider('YOUR_ETHEREUM_PROVIDER_URL');
const wallet = new ethers.Wallet('YOUR_PRIVATE_KEY', provider);

// Replace with the contract address and ABI of the Router contract
const routerAddress = 'YOUR_ROUTER_CONTRACT_ADDRESS';
const routerABI = [
  // Include the ABI of your Router contract here
];

// Create an instance of the Router contract
const routerContract = new ethers.Contract(routerAddress, routerABI, wallet);

// Function to fetch token prices
async function fetchTokenPrices() {
  try {
    const response = await axios.get('https://api.gmx.io/prices');
    const tokenPrices = response.data;

    // Prices are multiplied by (10 ** 30)
    console.log('Token Prices:', tokenPrices);
  } catch (error) {
    console.error('Error fetching token prices:', error);
  }
}

// Function to fetch actions
async function fetchActions(account, after) {
  try {
    // Construct the URL with optional query parameters
    const url = 'https://api.gmx.io/actions';
    const params = {};
    if (account) params.account = account;
    if (after) params.after = after;

    const response = await axios.get(url, { params });
    const actions = response.data;

    console.log('Actions:', actions);
  } catch (error) {
    console.error('Error fetching actions:', error);
  }
}

// Example usage
const tokenIn = { address: 'TOKEN_IN_ADDRESS' };
const tokenOut = { address: 'TOKEN_OUT_ADDRESS' };
const amountIn = ethers.utils.parseUnits('100', 18);
const minOut = ethers.utils.parseUnits('500', 18);
const receiver = 'RECEIVER_ADDRESS';

// Execute a swap
executeSwap(tokenIn, tokenOut, amountIn, minOut, receiver);

// Query available amounts
const vault = 'VAULT_ADDRESS';
queryAvailableAmounts(vault, tokenIn, tokenOut);

// Fetch token prices
fetchTokenPrices();

// Fetch actions with optional parameters
fetchActions('ACCOUNT_ADDRESS', 'START_AFTER_ID');
