import { useEffect, useState } from "react";
import { CalimeroSdk, WalletConnection } from "calimero-sdk";
import { getVoteResults, setPollOptions } from "../../utils/callMethods";

export interface Poll {
  question: string;
  options: string[];
}

export interface Option {
  option: string;
  count: number;
}

interface VotingComponentProps {
  contractCall: (option: string, calimero: CalimeroSdk | undefined) => void;
  calimero: CalimeroSdk | undefined;
  walletConnectionObject: WalletConnection | undefined
}

export default function VotingComponent(
  {contractCall, calimero, walletConnectionObject}: VotingComponentProps 
) {
  const [pollData, setPollData] = useState<Poll>({
          question: '',
          options: []
        });
  const [options, setOptions] = useState<Option[]>([]);

  useEffect(()=>{
    setPollOptions(setPollData,walletConnectionObject);
    getVoteResults(setOptions, walletConnectionObject);
  },[]);

  const createNewVote = async (option: string) => {
      await contractCall(option, calimero);
      await getVoteResults(setOptions, walletConnectionObject);
  };

  return (
    <div className="w-full px-10 py-10 flex justify-center">
      <div className="w-10/12 bg-black rounded-md px-4 py-4 text-white">
        <p>
          Poll question: <span className="font-black">{pollData?.question}</span>{" "}
        </p>
        <p className="pt-4 pb-4">Poll options:</p>
        <div className="grid grid-cols-3 w-full gap-x-4 gap-y-2">
          {pollData?.options.map((option: string, i: number) => {
            return (
              <button
                key={i}
                className="col-span-1 bg-white text-black rounded-md px-6 py-2 text-center cursor-pointer hover:bg-violet-700 transition duration-700 hover:text-white"
                onClick={() => createNewVote(option)}
              >
                {option}
              </button>
            );
          })}
        </div>
        <div className="pt-4">
          <p className="pt-4 pb-4">Poll results:</p>
          {options?.map((vote, i) => (
            <p key={i}>
              {vote.option}:
              <span className="font-black pl-2">{vote.count}</span>
            </p>
          ))}
        </div>
      </div>
    </div>
  );
}
