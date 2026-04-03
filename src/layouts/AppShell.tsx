import Banner from "../components/banner/Banner";
import Sidebar from "../components/sidebar/Sidebar";
import ContentArea from "../components/content/ContentArea";
import StatusBar from "../components/statusbar/StatusBar";

export default function AppShell() {
  return (
    <div class="grid h-screen grid-rows-[auto_1fr_auto] grid-cols-[auto_1fr]">
      <div class="col-span-2">
        <Banner />
      </div>
      <Sidebar />
      <ContentArea />
      <div class="col-span-2">
        <StatusBar />
      </div>
    </div>
  );
}
