import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import PageWrapper from "../../components/nh/pageWrapper/PageWrapper";
import GameCard from "../../components/nh/gameCard/GameCard";
import { Calimero, GameProps } from "..";
import GameBoard from "../../components/nh/gameBoard/GameBoard";
import { getGameData, makeAMoveMethod } from "../../utils/callMethods";
import { getGameStatus } from "../../utils/styleFunctions";
import translations from "../../constants/en.global.json";
import { CalimeroSdk, WalletConnection } from "calimero-sdk";
import { config } from "../../utils/calimeroSdk";
import useNear from "../../utils/useNear";

const contractName = process.env.NEXT_PUBLIC_CONTRACT_ID || "";
let walletConnectionObject: WalletConnection | undefined = undefined;
let calimero: Calimero | undefined = undefined;

export default function Game() {
  const router = useRouter();
  const { id } = router.query;
  const [gameStatus, setGameStatus] = useState<GameProps>();
  const [isSignedIn, setIsSignedIn] = useState(false);
  const {
    login,
    logout,
    register,
    registerStatus,
    nearSignedIn,
    setRegisterStatus,
  } = useNear();

  const signOut = () => {
    walletConnectionObject?.signOut();
    localStorage.removeItem("calimeroAccountId");
    setIsSignedIn(false);
  };

  const signIn = async () => {
    await walletConnectionObject?.requestSignIn({
      contractId: contractName,
      methodNames: ["make_a_move"],
    });
  };

  const makeMoveFunctionCall = async (id: number, squareId: number) => {
    await makeAMoveMethod(id, squareId, walletConnectionObject, signIn, router);
  };

  useEffect(() => {
    const init = async () => {
      calimero = await CalimeroSdk.init(config).connect();
      if (id) {
        getGameData(parseInt(id?.toString() || ""), setGameStatus, calimero);
      }
      walletConnectionObject = new WalletConnection(calimero, contractName);
      await walletConnectionObject.isSignedInAsync();
      localStorage.setItem("calimeroAccountId",walletConnectionObject.getAccountId());
    };
    if (nearSignedIn) {
      init();
    }
  }, [nearSignedIn, id]);

  useEffect(() => {
    const absolute = window.location.href.split("?");
    const url = absolute[0];
    router.replace(url);
  }, []);

  return (
    <PageWrapper
      isSignedIn={isSignedIn}
      title={translations.pages.indexPageTitle}
      currentPage={router.pathname}
      nearLogin={login}
      nearLogout={logout}
      calimeroLogout={signOut}
      gameRegister={register}
      status={registerStatus}
      setStatus={setRegisterStatus}
      nearSignedIn={nearSignedIn}
    >
      {gameStatus && id && (
        <div className="mt-10">
          <GameCard
            gameId={parseInt(id.toString())}
            playerA={gameStatus.playerA}
            playerB={gameStatus.playerB}
            status={getGameStatus(
                              gameStatus.status,
                              gameStatus.playerA,
                              walletConnectionObject?.getAccountId()
                            )}
            play={false}
          />
          {gameStatus.playerTurn == localStorage.getItem("nearAccountId") && (
            <div className="flex justify-center items-center mt-4 text-white text-base font-semibold">
              {translations.currentGamesPage.turnText}
            </div>
          )}
          <GameBoard
            gameData={gameStatus}
            gameId={parseInt(id.toString())}
            callMethod={(id, squareId) => makeMoveFunctionCall(id, squareId)}
          />
        </div>
      )}
    </PageWrapper>
  );
}
