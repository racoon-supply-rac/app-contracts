/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.25.2.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { ExecuteMsg, AssetInfo, Uint128, Decimal, Cw721Coin, Sg721Token, Coin, LoanTerms, InstantiateMsg, QueryMsg } from "./NFTLoan.types";
export interface NFTLoanReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<ConfigResponse>;
  borrowerInfo: ({
    borrower
  }: {
    borrower: string;
  }) => Promise<BorrowerInfoResponse>;
  collateralInfo: ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }) => Promise<CollateralInfoResponse>;
  collaterals: ({
    borrower,
    limit,
    startAfter
  }: {
    borrower: string;
    limit?: number;
    startAfter?: number;
  }) => Promise<CollateralsResponse>;
  allCollaterals: ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string[][];
  }) => Promise<AllCollateralsResponse>;
  offerInfo: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }) => Promise<OfferInfoResponse>;
  offers: ({
    borrower,
    limit,
    loanId,
    startAfter
  }: {
    borrower: string;
    limit?: number;
    loanId: number;
    startAfter?: string;
  }) => Promise<OffersResponse>;
  lenderOffers: ({
    lender,
    limit,
    startAfter
  }: {
    lender: string;
    limit?: number;
    startAfter?: string;
  }) => Promise<LenderOffersResponse>;
}
export class NFTLoanQueryClient implements NFTLoanReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.borrowerInfo = this.borrowerInfo.bind(this);
    this.collateralInfo = this.collateralInfo.bind(this);
    this.collaterals = this.collaterals.bind(this);
    this.allCollaterals = this.allCollaterals.bind(this);
    this.offerInfo = this.offerInfo.bind(this);
    this.offers = this.offers.bind(this);
    this.lenderOffers = this.lenderOffers.bind(this);
  }

  config = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  borrowerInfo = async ({
    borrower
  }: {
    borrower: string;
  }): Promise<BorrowerInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      borrower_info: {
        borrower
      }
    });
  };
  collateralInfo = async ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }): Promise<CollateralInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      collateral_info: {
        borrower,
        loan_id: loanId
      }
    });
  };
  collaterals = async ({
    borrower,
    limit,
    startAfter
  }: {
    borrower: string;
    limit?: number;
    startAfter?: number;
  }): Promise<CollateralsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      collaterals: {
        borrower,
        limit,
        start_after: startAfter
      }
    });
  };
  allCollaterals = async ({
    limit,
    startAfter
  }: {
    limit?: number;
    startAfter?: string[][];
  }): Promise<AllCollateralsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_collaterals: {
        limit,
        start_after: startAfter
      }
    });
  };
  offerInfo = async ({
    globalOfferId
  }: {
    globalOfferId: string;
  }): Promise<OfferInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      offer_info: {
        global_offer_id: globalOfferId
      }
    });
  };
  offers = async ({
    borrower,
    limit,
    loanId,
    startAfter
  }: {
    borrower: string;
    limit?: number;
    loanId: number;
    startAfter?: string;
  }): Promise<OffersResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      offers: {
        borrower,
        limit,
        loan_id: loanId,
        start_after: startAfter
      }
    });
  };
  lenderOffers = async ({
    lender,
    limit,
    startAfter
  }: {
    lender: string;
    limit?: number;
    startAfter?: string;
  }): Promise<LenderOffersResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      lender_offers: {
        lender,
        limit,
        start_after: startAfter
      }
    });
  };
}
export interface NFTLoanInterface extends NFTLoanReadOnlyInterface {
  contractAddress: string;
  sender: string;
  listCollaterals: ({
    comment,
    loanPreview,
    terms,
    tokens
  }: {
    comment?: string;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
    tokens: AssetInfo[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  modifyCollaterals: ({
    comment,
    loanId,
    loanPreview,
    terms
  }: {
    comment?: string;
    loanId: number;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawCollaterals: ({
    loanId
  }: {
    loanId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  makeOffer: ({
    borrower,
    comment,
    loanId,
    terms
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
    terms: LoanTerms;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  cancelOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  refuseOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawRefusedOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  acceptOffer: ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  acceptLoan: ({
    borrower,
    comment,
    loanId
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  repayBorrowedFunds: ({
    loanId
  }: {
    loanId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawDefaultedLoan: ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setOwner: ({
    owner
  }: {
    owner: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setFeeDestination: ({
    treasuryAddr
  }: {
    treasuryAddr: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setFeeRate: ({
    feeRate
  }: {
    feeRate: Decimal;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setListingCoins: ({
    listingFeeCoins
  }: {
    listingFeeCoins: Coin[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class NFTLoanClient extends NFTLoanQueryClient implements NFTLoanInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.listCollaterals = this.listCollaterals.bind(this);
    this.modifyCollaterals = this.modifyCollaterals.bind(this);
    this.withdrawCollaterals = this.withdrawCollaterals.bind(this);
    this.makeOffer = this.makeOffer.bind(this);
    this.cancelOffer = this.cancelOffer.bind(this);
    this.refuseOffer = this.refuseOffer.bind(this);
    this.withdrawRefusedOffer = this.withdrawRefusedOffer.bind(this);
    this.acceptOffer = this.acceptOffer.bind(this);
    this.acceptLoan = this.acceptLoan.bind(this);
    this.repayBorrowedFunds = this.repayBorrowedFunds.bind(this);
    this.withdrawDefaultedLoan = this.withdrawDefaultedLoan.bind(this);
    this.setOwner = this.setOwner.bind(this);
    this.setFeeDestination = this.setFeeDestination.bind(this);
    this.setFeeRate = this.setFeeRate.bind(this);
    this.setListingCoins = this.setListingCoins.bind(this);
  }

  listCollaterals = async ({
    comment,
    loanPreview,
    terms,
    tokens
  }: {
    comment?: string;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
    tokens: AssetInfo[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      list_collaterals: {
        comment,
        loan_preview: loanPreview,
        terms,
        tokens
      }
    }, fee, memo, funds);
  };
  modifyCollaterals = async ({
    comment,
    loanId,
    loanPreview,
    terms
  }: {
    comment?: string;
    loanId: number;
    loanPreview?: AssetInfo;
    terms?: LoanTerms;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      modify_collaterals: {
        comment,
        loan_id: loanId,
        loan_preview: loanPreview,
        terms
      }
    }, fee, memo, funds);
  };
  withdrawCollaterals = async ({
    loanId
  }: {
    loanId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_collaterals: {
        loan_id: loanId
      }
    }, fee, memo, funds);
  };
  makeOffer = async ({
    borrower,
    comment,
    loanId,
    terms
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
    terms: LoanTerms;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      make_offer: {
        borrower,
        comment,
        loan_id: loanId,
        terms
      }
    }, fee, memo, funds);
  };
  cancelOffer = async ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      cancel_offer: {
        global_offer_id: globalOfferId
      }
    }, fee, memo, funds);
  };
  refuseOffer = async ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      refuse_offer: {
        global_offer_id: globalOfferId
      }
    }, fee, memo, funds);
  };
  withdrawRefusedOffer = async ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_refused_offer: {
        global_offer_id: globalOfferId
      }
    }, fee, memo, funds);
  };
  acceptOffer = async ({
    globalOfferId
  }: {
    globalOfferId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_offer: {
        global_offer_id: globalOfferId
      }
    }, fee, memo, funds);
  };
  acceptLoan = async ({
    borrower,
    comment,
    loanId
  }: {
    borrower: string;
    comment?: string;
    loanId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_loan: {
        borrower,
        comment,
        loan_id: loanId
      }
    }, fee, memo, funds);
  };
  repayBorrowedFunds = async ({
    loanId
  }: {
    loanId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      repay_borrowed_funds: {
        loan_id: loanId
      }
    }, fee, memo, funds);
  };
  withdrawDefaultedLoan = async ({
    borrower,
    loanId
  }: {
    borrower: string;
    loanId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_defaulted_loan: {
        borrower,
        loan_id: loanId
      }
    }, fee, memo, funds);
  };
  setOwner = async ({
    owner
  }: {
    owner: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_owner: {
        owner
      }
    }, fee, memo, funds);
  };
  setFeeDestination = async ({
    treasuryAddr
  }: {
    treasuryAddr: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_fee_destination: {
        treasury_addr: treasuryAddr
      }
    }, fee, memo, funds);
  };
  setFeeRate = async ({
    feeRate
  }: {
    feeRate: Decimal;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_fee_rate: {
        fee_rate: feeRate
      }
    }, fee, memo, funds);
  };
  setListingCoins = async ({
    listingFeeCoins
  }: {
    listingFeeCoins: Coin[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_listing_coins: {
        listing_fee_coins: listingFeeCoins
      }
    }, fee, memo, funds);
  };
}