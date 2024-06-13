import Web3 from 'web3';
import contractABI from './DecentralizedVotingAppABI.json';
import axios from 'axios';

const CONTRACT_ADDRESS = process.env.REACT_APP_CONTRACT_ADDRESS;
const BACKEND_API_URL = process.env.REACT_APP_BACKEND_API_URL;

const web3 = new Web3(Web3.givenProvider || "http://localhost:7545");
const votingContract = new web3.eth.Contract(contractABI, CONTRACT_ADDRESS);


async function authenticateUser(username, password) {
    try {
        console.log('Authenticating user:', username);
        return true;
    } catch (error) {
        console.error('Error during authentication:', error);
        throw new Error(`Authentication failure: ${error.message}`);
    }
}

async function submitVote(voteDetails, userAccount) {
    try {
        if (!userAccount) {
            throw new Error('User account is not provided. Please ensure you are logged in and a valid account is available.');
        }
        const receipt = await votingContract.methods.vote(voteDetails.candidateId, voteDetails.vote).send({ from: userAccount });
        console.log('Vote submitted!', receipt);
    } catch (error) {
        console.error('Error submitting vote:', error);
        throw new Error(`Failed to submit vote: ${error.message}`);
    }
}

async function fetchVoteData() {
    try {
        const response = await axios.get(`${BACKEND_API_URL}/votes`);
        console.log('Fetched vote data:', response.data);
        return response.data;
    } catch (error) {
        console.error('Error fetching vote data:', error);
        throw new Error(`Failed to fetch vote data: ${
 error.response ? 
 (error.response.data.message || error.response.statusText) : 
 error.message}`);
    }
}

function displayVoteResults(voteResults) {
    console.log('Vote Results:', voteResults);
}

async function fetchCandidateVoteCount(candidateId) {
    try {
        const voteCount = await votingContract.methods.getVotesForCandidate(candidateId).call();
        console.log(`Vote count for candidate ${candidateId}:`, voteCount);
        return voteCount;
    } catch (error) {
        console.error(`Error fetching vote count for candidate ${candidateId}:`, error);
        throw new Error(`Failed to fetch vote count for candidate ${candidateId}: ${error.message}`);
    }
}

async function registerNewCandidate(candidateName, userAccount) {
    try {
        if (!userAccount) {
            throw new Error('User account is not provided for registering a new candidate. Please ensure you are logged in and a valid account is available.');
        }
        const receipt = await votingContract.methods.registerCandidate(candidateName).send({ from: userAccount });
        console.log(`New candidate "${candidateName}" registered!`, receipt);
    } catch (error) {
        console.error(`Error registering new candidate "${candidateName}":`, error);
        throw new Error(`Failed to register new candidate "${candidateName}": ${error.message}`);
    }
}

async function main() {
    try {
        const username = 'user';
        const password = 'password';

        const isAuthenticated = await authenticateUser(username, password);
        if (!isAuthenticated) {
            throw new Error('Authentication failed.');
        }

        const accounts = await web3.eth.getAccounts();
        if (accounts.length === 0) throw new Error("No accounts retrieved. Ensure Web3 is properly initialized and connected.");
        const userAccount = accounts[0];

        await registerNewCandidate('New Candidate', userAccount);
        
        const voteDetails = { candidateId: 'candidate1', vote: 1 };
        await submitVote(voteDetails, userAccount);

        await fetchCandidateVoteCount(voteDetails.candidateId);

        const voteResults = await fetchVoteData();
        displayVoteResults(voteResults);
    } catch (error) {
        console.error("An error occurred in the main flow:", error.message);
    }
}

main().catch(error => console.error("Unhandled Error:", error));