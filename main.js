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
        // The authentication logic should be implemented here
        return true;
    } catch (error) {
        console.error('Error during authentication:', error);
        return false;
    }
}

async function submitVote(voteDetails, userAccount) {
    try {
        if (!userAccount) {
            console.error('User account is not provided');
            return;
        }
        const receipt = await votingContract.methods.vote(voteDetails.candidateId, voteDetails.vote).send({ from: userAccount });
        console.log('Vote submitted!', receipt);
    } catch (error) {
        console.error('Error submitting vote:', error);
    }
}

async function fetchVoteData() {
    try {
        const response = await axios.get(`${BACKEND_API_URL}/votes`);
        console.log('Fetched vote data:', response.data);
        return response.data;
    } catch (error) {
        console.error('Error fetching vote data:', error);
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
    }
}

async function registerNewCandidate(candidateName, userAccount) {
    try {
        if (!userAccount) {
            console.error('User account is not provided for registering new candidate');
            return;
        }
        const receipt = await votingContract.methods.registerCandidate(candidateName).send({ from: userAccount });
        console.log(`New candidate "${candidateName}" registered!`, receipt);
    } catch (error) {
        console.error(`Error registering new candidate "${candidateName}":`, error);
    }
}

async function main() {
    const username = 'user';
    const password = 'password';

    const isAuthenticated = await authenticateUser(username, password);
    if (!isAuthenticated) {
        console.error('Authentication failed.');
        return;
    }

    const accounts = await web3.eth.getAccounts();
    const userAccount = accounts[0];

    await registerNewCandidate('New Candidate', userAccount);

    const voteDetails = { candidateId: 'candidate1', vote: 1 };
    await submitVote(voteDetails, userAccount);

    await fetchCandidateVoteCount(voteDetails.candidateId);

    const voteResults = await fetchVoteData();
    displayVoteResults(voteResults);
}

main().catch(console.error);