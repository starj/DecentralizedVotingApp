// SPDX-License-Identifier: MIT
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

    function createVote(string memory _description, uint[] calldata _candidateList) public {
        require(_candidateList.length > 0, "Candidate list must contain at least one candidate.");
        voteCount++;
        Vote storage newVote = votes[voteCount];
        newVote.id = voteCount;
        newVote.description = _description;
        newVote.exists = true;
        
        for (uint i = 0; i < _candidateList.length; i++) {
            require(_candidateList[i] != 0, "Candidate ID 0 is reserved and cannot be explicitly used.");
            newVote.candidateVotes[_candidateList[i]] = 0; 
            newVote.candidateList.push(_candidateList[i]);
        }

        emit VoteCreated(voteCount, _description);
    }

    function castVote(uint _voteId, uint _candidateId) public {
        require(votes[_voteId].exists, "Vote does not exist.");
        require(!voterHasVoted[msg.sender][_voteId], "You have already voted in this vote.");
        require(_isCandidateInVote(_voteId, _candidateId), "Candidate does not exist in this vote.");

        votes[_voteId].candidateVotes[_candidateId]++;
        voterHasVoted[msg.sender][_voteId] = true;

        emit Voted(_voteId, _candidateId, msg.sender);
    }

    function getVoteResults(uint _voteId) public view returns (uint[] memory, uint[] memory) {
        require(votes[_voteId].exists, "Vote does not exist.");
        uint[] memory ids = new uint[](votes[_voteId].candidateList.length);
        uint[] memory voteCounts = new uint[](votes[_voteId].candidateList.length);
        
        for (uint i = 0; i < votes[_voteId].candidateList.length; i++) {
            ids[i] = votes[_voteId].candidateList[i];
            voteCounts[i] = votes[_voteId].candidateVotes[ids[i]];
        }
        
        return (ids, voteCounts);
    }
    
    function _isCandidateInPropsosal(uint _voteId, uint _candidateId) private view returns (bool) {
        uint length = votes[_voteId].candidateList.length;
        for (uint i = 0; i < length; i++) {
            if (votes[_voteText>[i] == _candidateId) {
                return true;
            }
        }
        return false;
    }
}