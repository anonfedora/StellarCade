import React, { Suspense, lazy } from "react";
import { BrowserRouter, useNavigate } from "react-router-dom";
import GameLobby from "./pages/GameLobby";
import { RouteErrorBoundary } from "./components/v1/RouteErrorBoundary";
import ProfileSettings from "./pages/ProfileSettings";
import Portfolio from "./pages/Portfolio";
import { I18nProvider, useI18n } from "./i18n/provider";
import LocaleSwitcher from "./components/LocaleSwitcher";
import Breadcrumbs from "./components/BreadCrumbs";
import AppSidebar from "./components/v1/AppSidebar";
import NotificationCenter from "./components/v1/NotificationCenter";
import { ModalStackProvider } from "./components/v1/modal-stack";
import { FeatureFlagsProvider } from "./services/feature-flags";
import CommandPalette, { type Command } from "./components/v1/CommandPalette";

const DevContractCallSimulatorPanel = import.meta.env.DEV
  ? lazy(() =>
      import("./components/dev/ContractCallSimulatorPanel").then((module) => ({
        default: module.ContractCallSimulatorPanel,
      })),
    )
  : undefined;

const MAIN_CONTENT_ID = "main-content";

type AppRoute = "lobby" | "games" | "portfolio" | "profile";

const AppContent: React.FC = () => {
  const { t } = useI18n();
  const [route, setRoute] = React.useState<AppRoute>("lobby");
  const navigate = useNavigate();

  const commands: Command[] = [
    {
      id: "go-lobby",
      label: "Go to Lobby",
      description: "Open the game lobby",
      action: () => navigate("/"),
    },
    {
      id: "go-games",
      label: "Go to Games",
      description: "Open the games section",
      action: () => setRoute("games"),
    },
    {
      id: "go-profile",
      label: "Go to Profile Settings",
      description: "Open the profile settings page",
      action: () => navigate("/profile"),
    },
    {
      id: "go-portfolio",
      label: "Go to Portfolio",
      description: "Open wallet, rewards, and collectibles",
      action: () => setRoute("portfolio"),
    },
  ];

  return (
    <div className="app-container">
      <CommandPalette commands={commands} />
      <NotificationCenter />

      <a
        href={`#${MAIN_CONTENT_ID}`}
        className="skip-link"
        onClick={(event) => {
          const mainContent = document.getElementById(MAIN_CONTENT_ID);
          if (!mainContent) return;

          event.preventDefault();
          mainContent.focus();
          mainContent.scrollIntoView?.({ block: "start" });
        }}
      >
        Skip to main content
      </a>

      <AppSidebar currentRoute={route} onNavigate={setRoute} />

      <div className="app-main-layout">
        <header className="app-header">
          <div className="logo">{t("app.title")}</div>
          <LocaleSwitcher />
        </header>

        <Breadcrumbs />

        <main className="app-content" id={MAIN_CONTENT_ID} tabIndex={-1}>
          <RouteErrorBoundary>
            {route === "profile" ? (
              <ProfileSettings />
            ) : route === "portfolio" ? (
              <Portfolio
                onOpenWallet={() => setRoute("profile")}
                onBrowseRewards={() => setRoute("games")}
                onBrowseCollectibles={() => setRoute("games")}
              />
            ) : (
              <GameLobby />
            )}
          </RouteErrorBoundary>
        </main>

        <footer className="app-footer">
          <div className="footer-content">
            <p>{t("footer.copyright")}</p>

            <div className="footer-links">
              <a href="/terms">{t("footer.terms")}</a>
              <a href="/privacy">{t("footer.privacy")}</a>
            </div>
          </div>
        </footer>
      </div>

      {import.meta.env.DEV && DevContractCallSimulatorPanel ? (
        <Suspense fallback={null}>
          <DevContractCallSimulatorPanel />
        </Suspense>
      ) : null}
    </div>
  );
};

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <FeatureFlagsProvider>
        <I18nProvider>
          <ModalStackProvider>
            <AppContent />
          </ModalStackProvider>
        </I18nProvider>
      </FeatureFlagsProvider>
    </BrowserRouter>
  );
};

export { Drawer } from "./components/v1/Drawer";

export default App;
