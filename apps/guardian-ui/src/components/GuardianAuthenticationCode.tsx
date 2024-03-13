import React, { useState } from 'react';
import {
  Text,
  useTheme,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalCloseButton,
  Flex,
  Button,
} from '@chakra-ui/react';
import { useTranslation } from '@fedimint/utils';
import QRCode from 'qrcode.react';

const QR_CODE_SIZE = 256;

type GuardianAuthCode = {
  federationId: string;
  peerId: number;
  guardianName: string;
  password: string;
};

interface GuardianAuthenticationCodeProps {
  federationId: string;
  ourPeer: { id: number; name: string } | undefined;
}

export const GuardianAuthenticationCode: React.FC<
  GuardianAuthenticationCodeProps
> = ({ federationId, ourPeer }) => {
  const theme = useTheme();
  const { t } = useTranslation();
  const [isOpen, setIsOpen] = useState(false);
  const [qrValue, setQrValue] = useState<string>('');
  const [isAcknowledged, setIsAcknowledged] = useState(false);

  const calculateGuardianAuthenticationCode = () => {
    const password = sessionStorage.getItem('guardian-ui-key');
    const guardianAuthCode = {
      federationId,
      peerId: ourPeer?.id,
      guardianName: ourPeer?.name,
      password,
    } as GuardianAuthCode;

    return `guardian:authenticate?${JSON.stringify(guardianAuthCode)}`;
  };

  const handleOpen = () => {
    setQrValue(calculateGuardianAuthenticationCode());
    setIsOpen(true);
    setIsAcknowledged(false);
  };
  const handleClose = () => setIsOpen(false);

  const handleAcknowledge = () => {
    setIsAcknowledged(true);
  };

  return (
    <>
      <Button
        onClick={handleOpen}
        bg={theme.colors.red[500]}
        _hover={{ bg: theme.colors.red[600] }}
      >
        Authenticate as Guardian
      </Button>
      <Modal isOpen={isOpen} onClose={handleClose}>
        <ModalOverlay />
        <ModalContent minH='0'>
          <ModalHeader alignSelf='center'>
            {t('federation-dashboard.modal.guardian-authenticate')}
          </ModalHeader>
          <ModalCloseButton />
          <ModalBody pb={6}>
            {!isAcknowledged ? (
              <Flex
                justifyContent='center'
                alignItems='center'
                direction='column'
              >
                <Text mb={4}>
                  {t(
                    'federation-dashboard.danger-zone.guardian-warning-message'
                  )}
                </Text>
                <Button colorScheme='blue' onClick={handleAcknowledge}>
                  {t('federation-dashboard.danger-zone.guardian-acknowledge')}
                </Button>
              </Flex>
            ) : (
              <Flex
                justifyContent='center'
                alignItems='center'
                direction='column'
              >
                <QRCode
                  value={qrValue}
                  size={QR_CODE_SIZE}
                  style={{
                    width: '100%',
                    height: 'auto',
                    maxWidth: QR_CODE_SIZE,
                  }}
                />
              </Flex>
            )}
          </ModalBody>
        </ModalContent>
      </Modal>
    </>
  );
};
