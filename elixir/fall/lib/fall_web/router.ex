defmodule FallWeb.Router do
  use FallWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", FallWeb do
    pipe_through :browser # использовать стек браузера по умолчанию

    get "/", PageController, :index
    get "/welcome", FallController, :welcome
    get "/fall", FallController, :faller
    post "/fall", FallController, :faller
  end

  # Other scopes may use custom stacks.
  # scope "/api", FallWeb do
  #   pipe_through :api
  # end
end
