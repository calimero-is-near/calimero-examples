import { useEffect, useState } from "react";
import Square from "./Square";
import NotificationCard from "../notificationCard/NotificationCard";

export interface GameProps {
  boardStatus: string[];
  playerA: string;
  playerB: string;
  playerTurn: string;
  status: string;
}

interface IProps {
  gameData: GameProps;
  gameId: number;
  callMethod: (id: number, squareId: number) => void;
  updateBoard: () => void;
}

export default function GameBoard({
  gameData,
  gameId,
  callMethod,
  updateBoard,
}: IProps) {
  const [squares, setSquares] = useState<string[]>(Array(9).fill(null));
  const [ended, setEnded] = useState<string>("");
  const [color, setColor] = useState<string>("bg-white");
  const [show, setShow] = useState(false);
  const [title, setTitle] = useState("");
  const [subtitle, setSubtitle] = useState("");

  useEffect(() => {
    setBoard();
  }, [gameData]);

  const setBoard = async () => {
    setSquares(gameData.boardStatus);
    if (gameData.playerA === gameData.playerTurn) {
      setColor("hover:bg-gray-500");
    } else {
      setColor("hover:bg-nh-testnet");
    }
    setEnded(gameData.status);
  };
  const makeMove = async (index: number) => {
    const loggedUser = localStorage.getItem("nearAccountId");
    if (gameData.playerTurn !== loggedUser) {
      setShow(true);
      setTitle("Not your turn");
      setSubtitle("Wait for other player to make their move!");
    } else {
      setShow(true);
      setTitle("Move has been made");
      setSubtitle("Please wait for Blockchain to save your data!");
      await callMethod(gameId, index);
      setShow(false);
      updateBoard();
    }
  };

  return (
    <>
      <div className="absolute top-20 right-8">
        <NotificationCard
          title={title}
          subtitle={subtitle}
          show={show}
          setShow={setShow}
        />
      </div>
      <div className="px-10 py-6 flex justify-center gap-x-4">
        <div className="grid grid-cols-3 gap-x-4 gap-y-4">
          {Array(9)
            .fill(null)
            .map((_, i) => {
              return (
                <div className="col-span-1" key={i}>
                  <Square
                    key={i}
                    ended={ended}
                    color={color}
                    playerTurn={gameData.playerTurn}
                    playerA={gameData.playerA}
                    onClick={() => makeMove(i)}
                    value={squares[i]}
                  />
                </div>
              );
            })}
        </div>
      </div>
    </>
  );
}
