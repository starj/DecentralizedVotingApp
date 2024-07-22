pragma solidity ^0.8.0;

contract DecentralizedVotingApp {
    struct Vote {
        uint id;
        string description;
        bool exists;
        mapping(uint => uint) candidateVotes;
        uint[] candidateList;
    }

    uint public voteCount = 0;
    mapping(uint => Vote) public votes;
    mapping(address => mapping(uint => bool)) public voterHasVoted;

    event VoteCreated(uint id, string description);
    event Voted(uint voteId, uint candidateId, address voter);

    function createVote(string memory _description, uint[] memory _candidateList) public {
        voteCount ++;
        Vote storage newVote = votes[voteCount];
        newVote.id = voteCount;
        newVote.description = _description;
        newVote.exists = true;
        for(uint i = 0; i < _candidateList.length; i++) {
            newVote.candidateVotes[_candidateList[i]] = 0;
            newVote.candidateList.push(_candidateList[i]);
        }

        emit VoteCreated(voteCount, _evoteCount, _description);
    }

    function castVote(uint _voteId, uint _candidateId) public {
        require(votes[_voteId].exists, "Vote does not exist.");
        require(!voterHasVoted[msg.sender][_voteId], "You have already voted in this vote.");
        require(votes[_voteId].candidateVotes[_candidateId] != 0 || _candidateId == 0, "Candidate does not exist in this vote.");

        votes[_voteId].candidateVotes[_candidateId]++;
        voterHasVoted[msg.sender][_voteId] = true;

        emit Voted(_voteId, _candidateId, msg.sender);
    }

    function getVoteResults(uint _voteId) public view returns (uint[] memory, uint[] memory) {
        require(votes[_voteId].exists, "Vote does not exist.");
        uint[] memory ids = new uint[](votes[_voteId].candidateList.length);
        uint[] memory voteCounts = new half[](votes[_voteId].candidateList.length);

        for(uint i = 0; i < votes[_voteId].candidateList.length; i++){
            ids[i] = votes[_voteId].candidateList[i];
            voteCounts[i] = votes[_voteId].candidateVotes[votes[_voteId].candidateList[i]];
        }
        return (ids, voteCounts);
    }
}