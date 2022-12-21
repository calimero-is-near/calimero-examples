import { useRouter } from "next/router";
import { useState } from "react";
import PageWrapper from "../components/nh/pageWrapper/PageWrapper";

export default function Dashboard() {
  const router = useRouter();
  const [logged, setLogged] = useState<boolean>(true);
  return (
    <PageWrapper title={"PropUrl"} currentPage={router.pathname}>
      <div className="text-white">START GAME POPUP ON THIS CLICK</div>
    </PageWrapper>
  );
}
