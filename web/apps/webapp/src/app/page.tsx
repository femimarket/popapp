"use client"

import React, {useEffect, useMemo, useState} from "react"
import { ArrowDown, ArrowUp, ChevronDown } from "lucide-react"
import { EyeIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
import {Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Switch } from "@/components/ui/switch"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import TradingViewWidget from "@/app/chart";
import {appChain} from "@/lib/utils";
import { useAccount  } from "graz"
import {useAccounts, useCreateAccountMonarchy} from "@abstract-money/react";
import {pop} from "@/app/_generated/generated-abstract";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog"
import { ArrowUpRight, TrendingUp, Wallet, X } from "lucide-react"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table"

type Order = {
  id: number;
  asset: { quote:string, base:string };
  type: string;
  amount: number;
  // expiry: string;
}



export default function Component() {
  const [isBinaryOption, setIsBinaryOption] = useState(false)
  const [fundingAmount, setFundingAmount] = useState(1000)
  const [balance, setBalance] = useState(100000)
  const [isOpen, setIsOpen] = useState(false)
  const [selectedAsset, setSelectedAsset] = useState({ base: "BTC", quote: "USD" })
  const [tradeAmount, setTradeAmount] = useState(0)
  const [tradeType, setTradeType] = useState("buy")
  const [orders, setOrders] = useState<Order[]>([])

  const handleFund = () => {
    alert(`Funding successful! You've invested ${fundingAmount} ATOM coins.`)
    setIsOpen(false)
  }

  const { chainName, chainId } = appChain
  const { data: cosmosAccount } = useAccount({ chainId })

  const { mutate: createAccount, isLoading: isCreating, isSuccess: isAccountCreated } = useCreateAccountMonarchy({
    chainName,
  })
  const { data: accounts, isLoading: isLoadingAccounts, refetch: refetchAccounts } = useAccounts({
    args: {
      owner: cosmosAccount?.bech32Address ?? "",
      chains: [chainName],
    },
    query: {
      enabled: !!cosmosAccount?.bech32Address,
    },
  })

  useEffect(() => {
    let intervalId: NodeJS.Timeout | null = null;

    const fetchAccounts = async () => {
      if (isAccountCreated) {
        try {
          await refetchAccounts();
        } catch (error) {
          console.error("Error fetching accounts:", error);
        }
      }
    };

    if (isAccountCreated) {
      fetchAccounts(); // Fetch immediately when account is created
      intervalId = setInterval(fetchAccounts, 5000); // Then fetch every 5 seconds
    }

    return () => {
      if (intervalId) {
        clearInterval(intervalId);
      }
    };
  }, [isAccountCreated, refetchAccounts]);

  const createAbstractAccount = async () => {
    if (!cosmosAccount) {
      return
    }
    createAccount({
      args: { name: "Felipe test account", owner: cosmosAccount.bech32Address, installModules: [
          {
            module: {
              name: "pop",
              namespace: "pop",
              version: "latest"
            }
          },
          {
            module: {
              name: "slinky",
              namespace: "pop",
              version: "latest"
            }
          }
        ] },
      fee: "auto",
    })
  }

  const userAccountId = useMemo(() => {
    if (!accounts?.length) return undefined

    return accounts[accounts.length - 1]
  }, [accounts])




  const { data: popConfig } = pop.queries.useConfig({
    accountId: userAccountId,
    chainName: appChain.chainName
  })

  const {data:tradeData} = pop.queries.useTradeQuery({
    accountId: userAccountId,
    chainName: appChain.chainName,
    args: {
      base:"BTC",
      quote:"USD"
    }
  })

  const {data:pairsData} = pop.queries.usePairsQuery({
    accountId: userAccountId,
    chainName: appChain.chainName,
  })

  const assets = useMemo(() => {
    if (pairsData === undefined) return []
    return pairsData.pairs.currency_pairs
  }, [pairsData])

  const trade = async () => {
    console.log("creating abstract account")
    handleTrade()
    await createAbstractAccount()
  }

  console.log(userAccountId,"userAccountId",popConfig,"tradeData", tradeData,pairsData)

  const AssetList = () => (
    <div className="space-y-2">
      <Label htmlFor="limit-asset">Asset</Label>
      <Select
        value={`${selectedAsset.base}/${selectedAsset.quote}`}
        onValueChange={(value) => {
          const [base, quote] = value.split('/')
          setSelectedAsset({base, quote})
        }}
      >
        <SelectTrigger id="limit-asset">
          <SelectValue placeholder="Select asset"/>
        </SelectTrigger>
        <SelectContent>
          {assets.map((asset) => (
            <SelectItem key={`${asset.base}/${asset.quote}`} value={`${asset.base}/${asset.quote}`}>
              {`${asset.base}/${asset.quote}`}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  )

  const handleTrade = () => {
    const amount = tradeAmount
    if (isNaN(amount) || amount <= 0 || amount > balance) {
      alert("Invalid trade amount")
      return
    }

    // Add the user's order
    const userOrder: Order = {
      id: Date.now(),
      asset: selectedAsset,
      type: tradeType,
      amount: amount,
      // expiry: expiry
    }

    // Generate a random trader order
    const randomOrder: Order = {
      id: Date.now() + 1,
      asset: assets[Math.floor(Math.random() * assets.length)],
      type: Math.random() > 0.5 ? "call" : "put",
      amount: Math.floor(Math.random() * 10000) + 1000,
      // expiry: ["5m", "15m", "1h"][Math.floor(Math.random() * 3)]
    }

    setOrders(prevOrders => [...prevOrders,randomOrder].slice(-2))
    setBalance(prevBalance => prevBalance - amount)
    setTradeAmount(0)
  }

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>

      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="text-2xl font-bold">Fund Profitable Trader</DialogTitle>
          <DialogDescription>Invest in a proven trading strategy</DialogDescription>
        </DialogHeader>
        <Card className="border-0 shadow-none">
          <CardContent className="p-0">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <Wallet className="w-5 h-5 text-green-500"/>
                  <span className="font-semibold">Trader Balance:</span>
                </div>
                <span>50,000 ATOM</span>
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <TrendingUp className="w-5 h-5 text-blue-500"/>
                  <span className="font-semibold">30-Day ROI:</span>
                </div>
                <span className="text-green-500">+12.5%</span>
              </div>
              <div className="flex items-center justify-between">
                <div className="flex items-center space-x-2">
                  <ArrowUpRight className="w-5 h-5 text-purple-500"/>
                  <span className="font-semibold">Win Rate:</span>
                </div>
                <span>68%</span>
              </div>
              <div className="space-y-2">
                <Label htmlFor="fundingAmount">Funding Amount (ATOM)</Label>
                <Input
                  id="fundingAmount"
                  type="number"
                  value={fundingAmount}
                  onChange={(e) => setFundingAmount(Number(e.target.value))}
                  min={1}
                  max={10000}
                />
              </div>
            </div>
          </CardContent>
          <CardFooter className="p-0 pt-4">
            <Button onClick={handleFund} className="w-full">
              Fund Trader with {fundingAmount} ATOM
            </Button>
          </CardFooter>
        </Card>
        <Button
          className="absolute right-4 top-4"
          variant="ghost"
          size="icon"
          onClick={() => setIsOpen(false)}
        >
          <X className="h-4 w-4"/>
          <span className="sr-only">Close</span>
        </Button>
      </DialogContent>
      <div className="flex flex-col h-screen bg-background text-foreground">
        <header className="flex justify-between items-center p-4 border-b">
          <h1 className="text-2xl font-bold">Proof of Profit Protocol</h1>

          <div className="flex items-center gap-4">
            <span className="font-semibold">Simulated Capital:</span>
            <span className="text-xl font-bold">$100,000</span>
            <Button
              variant="outline"
              size="icon"
              className="relative"
              onClick={() => {}}
            >
              <EyeIcon className="h-4 w-4" />
              {orders.length > 0 && (
                <span className="absolute top-0 right-0 -mt-1 -mr-1 px-2 py-1 text-xs font-bold leading-none text-primary-foreground bg-primary rounded-full">
              {orders.length}
            </span>
              )}
            </Button>
          </div>

        </header>
        <div className="flex flex-1 overflow-hidden">
          <Card className="w-80 m-4 flex flex-col">
            <CardHeader>
              <CardTitle>Trading Panel</CardTitle>
              <CardDescription>Execute your trades here</CardDescription>
            </CardHeader>
            <CardContent className="overflow-auto">
              <div className="mt-auto">
                <Tabs defaultValue="market" className="w-full">
                  <TabsList className="grid w-full grid-cols-2">
                    <TabsTrigger value="market">Market</TabsTrigger>
                    <TabsTrigger value="limit">Limit</TabsTrigger>
                  </TabsList>
                  <TabsContent value="market">
                    <div className="space-y-4">
                      <AssetList />
                      <div className="flex items-center space-x-2">
                        <Label htmlFor="binary-option">Binary Option</Label>
                        <Switch
                          id="binary-option"
                          checked={isBinaryOption}
                          onCheckedChange={setIsBinaryOption}
                        />
                      </div>
                      {isBinaryOption ? (
                        <>
                          <div className="space-y-2">
                            <Label htmlFor="direction">Direction</Label>
                            <Select>
                              <SelectTrigger id="direction">
                                <SelectValue placeholder="Select direction"/>
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="up">
                                  <div className="flex items-center">
                                    <ArrowUp className="mr-2 h-4 w-4 text-green-500"/>
                                    Up
                                  </div>
                                </SelectItem>
                                <SelectItem value="down">
                                  <div className="flex items-center">
                                    <ArrowDown className="mr-2 h-4 w-4 text-red-500"/>
                                    Down
                                  </div>
                                </SelectItem>
                              </SelectContent>
                            </Select>
                          </div>
                          <div className="space-y-2">
                            <Label htmlFor="expiry">Expiry</Label>
                            <Select>
                              <SelectTrigger id="expiry">
                                <SelectValue placeholder="Select expiry"/>
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="5m">5 minutes</SelectItem>
                                <SelectItem value="15m">15 minutes</SelectItem>
                                <SelectItem value="1h">1 hour</SelectItem>
                              </SelectContent>
                            </Select>
                          </div>
                        </>
                      ) : (
                        <div className="space-y-2">
                          <Label htmlFor="side">Side</Label>
                          <Select>
                            <SelectTrigger id="side">
                              <SelectValue placeholder="Select side"/>
                            </SelectTrigger>
                            <SelectContent>
                              <SelectItem value="buy">Buy</SelectItem>
                              <SelectItem value="sell">Sell</SelectItem>
                            </SelectContent>
                          </Select>
                        </div>
                      )}
                      <div className="space-y-2">
                        <Label htmlFor="amount">Amount</Label>
                        <Input id="amount" placeholder="Enter amount" type="number"
                               value={tradeAmount}
                               onChange={(e) => setTradeAmount(parseFloat(e.target.value))}/>
                      </div>
                      <Button className="w-full" onClick={trade}>
                        {isBinaryOption ? "Place Binary Option" : "Place Market Order"}
                      </Button>
                      <DialogTrigger asChild>
                        <Button className="w-full" onClick={() => {
                        }}>
                          APE 🚀🚀
                        </Button>
                      </DialogTrigger>


                    </div>
                  </TabsContent>
                  <TabsContent value="limit">
                    <div className="space-y-4">
                      <AssetList />
                      <div className="space-y-2">
                        <Label htmlFor="limit-side">Side</Label>
                        <Select>
                          <SelectTrigger id="limit-side">
                            <SelectValue placeholder="Select side"/>
                          </SelectTrigger>
                          <SelectContent>
                            <SelectItem value="buy">Buy</SelectItem>
                            <SelectItem value="sell">Sell</SelectItem>
                          </SelectContent>
                        </Select>
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="limit-price">Limit Price</Label>
                        <Input id="limit-price" placeholder="Enter limit price" type="number"/>
                      </div>
                      <div className="space-y-2">
                        <Label htmlFor="limit-amount">Amount</Label>
                        <Input id="limit-amount" placeholder="Enter amount" type="number"/>
                      </div>
                      <Button className="w-full">Place Limit Order</Button>
                    </div>
                  </TabsContent>
                </Tabs>
              </div>
            </CardContent>
          </Card>
          <div className="flex-1 p-4 overflow-hidden">
            <div className="h-4/6">
              <TradingViewWidget asset={"PYTH:BTCUSD"}/>
            </div>
            <Card className="mt-4 m-0">
              <CardHeader>
                <CardTitle>Trading Orders</CardTitle>
                <CardDescription>Recent orders from all traders</CardDescription>
              </CardHeader>
              <CardContent>
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Asset</TableHead>
                      <TableHead>Type</TableHead>
                      <TableHead>Amount</TableHead>
                      <TableHead>Expiry</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {orders.map((order) => (
                      <TableRow key={order.id}>
                        <TableCell>{order.asset.base}/{order.asset.quote}</TableCell>
                        <TableCell className="capitalize">{order.type}</TableCell>
                        <TableCell>${order.amount.toFixed(2)}</TableCell>
                        {/*<TableCell>{order.expiry}</TableCell>*/}
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </CardContent>
            </Card>

          </div>
        </div>
      </div>

    </Dialog>
  )
}
